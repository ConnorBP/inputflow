//! KMBox plugin for inputflow.
//! Controls user input over serial interface to KMBox device.

use ::std::time::Duration;

use dataview::PodMethods;
use inputflow::prelude::*;
use serialport::SerialPort;
use format_bytes::format_bytes;

mod args;
pub mod keycodes;

struct KMBoxPluginRoot {
    controller: InputFlowKMBox,
}

impl KMBoxPluginRoot {
    pub fn new(args: args::Args) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(KMBoxPluginRoot {
            controller: InputFlowKMBox {
                port: serialport::new(args.com_port, args.baud_rate)
                        .timeout(Duration::from_millis(args.timeout_ms))
                        .open()?,
            }
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
        todo!()
    }

    #[doc = r" Releases a key that was set to down previously"]
    fn send_key_up(&mut self, key: KeyboardKey) -> Result<()> {
        todo!()
    }

    #[doc = r" Presses a key and lets it go all in one for when users do not care about specific timings"]
    fn press_key(&mut self, key: KeyboardKey) -> Result<()> {
        todo!()
    }

    #[doc = r" clears all active pressed keys. Useful for cleaning up multiple keys presses in one go."]
    #[doc = r" Ensures that keyboard writer is set back into a neutral state."]
    fn clear_keys(&mut self) -> Result<()> {
        todo!()
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
            MouseButton::Left => {
                self.km_set_left(1)
            },
            _=> {Err(InputFlowError::Parameter)}
        }
    }

    #[doc = r" Releases a mouse button that was set to down previously"]
    fn send_button_up(&mut self, button: MouseButton) -> Result<()> {
        match button {
            MouseButton::Left => {
                self.km_set_left(0)
            },
            _=> {Err(InputFlowError::Parameter)}
        }
    }

    #[doc = r" Presses a  mouse button and lets it go all in one for when users do not care about specific timings"]
    fn click_button(&mut self, button: MouseButton) -> Result<()> {
        
        let cmd = match button {
            MouseButton::Left => {
                //format_bytes!(b"km.click({})\r\n", button_id)
            },
            _=> {return Err(InputFlowError::Parameter);}
        };

         
        Ok(())
    }

    #[doc = r" clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go."]
    #[doc = r" Ensures that mouse writer is set back into a neutral state."]
    fn clear_buttons(&mut self) -> Result<()> {
        Ok(())
    }

    #[doc = r" Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically"]
    fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        let cmd = format_bytes!(b"km.move({},{})\r\n", x,y);
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
extern "C" fn create_plugin(lib: &CArc<cglue::trait_group::c_void>, args: *const std::ffi::c_char) -> Result<PluginInnerArcBox<'static>> {
    Ok(trait_obj!(
        (
            KMBoxPluginRoot::new(
                args::parse_args(args).map_err(|e| {

                    InputFlowError::Parameter
                })?
            ).map_err(|e| {

                InputFlowError::Loading
            })?,
            lib.clone()
        ) as PluginInner
    ))
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
