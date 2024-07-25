//! KMBox plugin for inputflow.
//! Controls user input over serial interface to KMBox device.

use std::time::Duration;

use dataview::PodMethods;
use format_bytes::format_bytes;
use inputflow::prelude::*;
use keycodes::KMBoxKeyboardKeyCode;
use serialport::{SerialPort, SerialPortType, UsbPortInfo};

mod args;
pub mod keycodes;

struct KMBoxPluginRoot {
    controller: InputFlowKMBox,
}

impl KMBoxPluginRoot {
    pub fn new(args: args::Args) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        log::info!(
            "Initializing KMBox plugin with config {}",
            ron::to_string(&args)?
        );

        let mut port_path = args.com_port;

        if args.auto_select {
            let ports = serialport::available_ports()?;

            for port in ports {
                log::trace!(
                    "Found serial port {} : {:?}",
                    port.port_name,
                    port.port_type
                );

                match port.port_type {
                    SerialPortType::UsbPort(UsbPortInfo {
                        product: Some(product_name),
                        ..
                    }) => {
                        if product_name.starts_with(&args.device_name) {
                            log::info!(
                                "Automatically loaded port {} from device {}",
                                port.port_name,
                                product_name
                            );
                            port_path = port.port_name;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(KMBoxPluginRoot {
            controller: InputFlowKMBox {
                port: serialport::new(port_path, args.baud_rate)
                    .timeout(Duration::from_millis(args.timeout_ms))
                    .open()?,
            },
        })
    }
}

impl<'a> PluginInner<'a> for KMBoxPluginRoot {
    type BorrowedType = Fwd<&'a mut InputFlowKMBox>;

    type OwnedType = InputFlowKMBox;
    type OwnedTypeMut = InputFlowKMBox;

    fn borrow_features(&'a mut self) -> Self::BorrowedType {
        self.controller.forward_mut()
    }

    fn into_features(self) -> Self::OwnedType {
        self.controller
    }

    fn mut_features(&'a mut self) -> &'a mut Self::OwnedTypeMut {
        &mut self.controller
    }
}

#[derive(Debug)]
pub struct InputFlowKMBox {
    pub port: Box<dyn SerialPort>,
}

impl InputFlowKMBox {
    /// calls km.left() with value to set current left click
    /// 1 = set down
    /// 0 = release
    pub fn km_set_left(&mut self, is_down: i32) -> Result<()> {
        let cmd = format_bytes!(b"km.left({})\r\n", is_down);

        self.port.write(cmd.as_bytes()).map_err(|e| {
            // log serial failure details if logging is enabled
            log::warn!("command km.left({is_down}) \"{cmd:?}\" failed: {e:?}.");
            // return error to result as InputFlowError type.
            InputFlowError::SendError
        })?;
        Ok(())
    }

    pub fn km_set_key(&mut self, key: KeyboardKey, is_down: bool) -> Result<()> {
        let km_key = KMBoxKeyboardKeyCode::try_from(key)?;

        let cmd = if is_down {
            format_bytes!(b"km.down({})\r\n", km_key)
        } else {
            format_bytes!(b"km.up({})\r\n", km_key)
        };

        self.port.write(cmd.as_bytes()).map_err(|e| {
            // log serial failure details if logging is enabled
            log::warn!("command km.down/km.up \"{cmd:?}\" failed: {e:?}.");
            // return error to result as InputFlowError type.
            InputFlowError::SendError
        })?;
        Ok(())
    }

    pub fn km_press_key(&mut self, key: KeyboardKey) -> Result<()> {
        let km_key = KMBoxKeyboardKeyCode::try_from(key)?;

        // press key command with some timing variation
        let cmd = format_bytes!(b"km.press({},15,50)\r\n", km_key);

        self.port.write(cmd.as_bytes()).map_err(|e| {
            // log serial failure details if logging is enabled
            log::warn!("command km.press({km_key}) \"{cmd:?}\" failed: {e:?}.");
            // return error to result as InputFlowError type.
            InputFlowError::SendError
        })?;
        Ok(())
    }
}

impl Loadable for InputFlowKMBox {
    fn name(&self) -> abi_stable::std_types::RString {
        "inputflow_kmbox".into()
    }

    fn capabilities(&self) -> u8 {
        IF_PLUGIN_HEAD.features.bits()
    }
}

impl KeyboardWriter for InputFlowKMBox {
    #[doc = r"Sends keyboard press down event"]
    fn send_key_down(&mut self, key: KeyboardKey) -> Result<()> {
        self.km_set_key(key, true)
    }

    #[doc = r" Releases a key that was set to down previously"]
    fn send_key_up(&mut self, key: KeyboardKey) -> Result<()> {
        self.km_set_key(key, false)
    }

    #[doc = r" Presses a key and lets it go all in one for when users do not care about specific timings"]
    fn press_key(&mut self, key: KeyboardKey) -> Result<()> {
        self.km_press_key(key)
    }

    #[doc = r" clears all active pressed keys. Useful for cleaning up multiple keys presses in one go."]
    #[doc = r" Ensures that keyboard writer is set back into a neutral state."]
    fn clear_keys(&mut self) -> Result<()> {
        // TODO: Add a currently pressed keys map and recursively set them unpressed
        log::info!("kmbox clear_keys not implemented yet...");
        Ok(())
    }
}

/// Takes in an inputflow mouse button and tries to
/// convert it to the equivilent kmbox button id
fn mouse_button_to_km(button: MouseButton) -> Option<u32> {
    Some(match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 3,
        MouseButton::XButton1 => 4,
        MouseButton::XButton2 => 5,
        _ => {
            return None;
        }
    })
}

impl MouseWriter for InputFlowKMBox {
    #[doc = r" Sends mouse button press down event"]
    fn send_button_down(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => self.km_set_left(1),
            _ => Err(InputFlowError::Parameter),
        }
    }

    #[doc = r" Releases a mouse button that was set to down previously"]
    fn send_button_up(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => self.km_set_left(0),
            _ => Err(InputFlowError::Parameter),
        }
    }

    #[doc = r" Presses a  mouse button and lets it go all in one for when users do not care about specific timings"]
    fn click_button(&mut self, button: MouseButton) -> Result<()> {
        let Some(km_button) = mouse_button_to_km(button) else {
            return Err(InputFlowError::InvalidKey);
        };

        let cmd = match button {
            MouseButton::Left => {
                format_bytes!(b"km.click({})\r\n", km_button)
            }
            _ => {
                return Err(InputFlowError::Parameter);
            }
        };

        // TODO: find anything other than km.click so that it may have some human-like delay rather than instantanious clicks
        self.port.write(cmd.as_bytes()).map_err(|e| {
            // log serial failure details if logging is enabled
            log::warn!("command km.click({button:?}) \"{cmd:?}\" failed: {e:?}.");
            // return error to result as InputFlowError type.
            InputFlowError::SendError
        })?;

        Ok(())
    }

    #[doc = r" clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go."]
    #[doc = r" Ensures that mouse writer is set back into a neutral state."]
    fn clear_buttons(&mut self) -> Result<()> {
        Ok(())
    }

    #[doc = r" Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically"]
    fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        let cmd = format_bytes!(b"km.move({},{})\r\n", x, y);
        self.port.write(cmd.as_bytes()).map_err(|e| {
            // log serial failure details if logging is enabled
            log::warn!("command km.move({x},{y}) \"{cmd:?}\" failed: {e:?}.");
            // return error to result as InputFlowError type.
            InputFlowError::SendError
        })?;
        Ok(())
    }
}

// =================================================================================================================
// =================================== CGlue Plugin init and Header definitions ====================================
// =================================================================================================================

cglue_impl_group!(InputFlowKMBox, ControllerFeatures,{KeyboardWriter, MouseWriter}, {KeyboardWriter, MouseWriter} );

/// Exposed interface that is called by the user of the plugin to instantiate it
#[allow(improper_ctypes_definitions)] // the linter is being stupid and not noticing the repr(u8)
extern "C" fn create_plugin(
    lib: &CArc<cglue::trait_group::c_void>,
    args: *const std::ffi::c_char,
) -> Result<PluginInnerArcBox<'static>> {
    env_logger::builder()
        // .filter_level(log::LevelFilter::Info)
        .init();
    Ok(trait_obj!((
        KMBoxPluginRoot::new(args::parse_args(args).map_err(|e| {
            log::error!("Invalid parameters were passed to inputflow_kmbox: {e:?}.");
            InputFlowError::Parameter
        })?)
        .map_err(|e| {
            log::error!("Failed to load KMBox device: {e:?}.");
            InputFlowError::Loading
        })?,
        lib.clone()
    ) as PluginInner))
}

/// Static plugin header values defining the plugin's capabilities
#[no_mangle]
pub static IF_PLUGIN_HEAD: PluginHeader = PluginHeader {
    features: FeatureSupport::from_bits_retain(
        FeatureSupport::WRITE_KEYBOARD.bits() | FeatureSupport::WRITE_MOUSE.bits(),
    ),
    layout: ROOT_LAYOUT,
    create: create_plugin,
};
