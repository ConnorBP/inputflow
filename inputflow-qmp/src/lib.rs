use std::io::Write;
use std::net::{TcpStream, SocketAddr};
use std::str;
use serde_json::json;
use inputflow::prelude::*;
use log::error;

mod args;

/// The plugin that handles the input flow functionality.
#[derive(Default)]
struct InputFlowQmp {
    stream: Option<TcpStream>,
    connected: bool,
}

impl Loadable for InputFlowQmp {
    fn name(&self) -> abi_stable::std_types::RString {
        "inputflow_qmp".into()
    }

    fn capabilities(&self) -> u8 {
        IF_PLUGIN_HEAD.features.bits()
    }
}

impl InputFlowQmp {
    /// Connect to the specified address and port.
    fn connect(&mut self, address: &str, port: u32) -> bool {
        if self.connected {
            println!("Connection is already open");
            return true;
        }

        let socket_address: SocketAddr = format!("{}:{}", address, port)
            .parse()
            .expect("Invalid address format");

        match TcpStream::connect(socket_address) {
            Ok(stream) => {
                self.stream = Some(stream);
                self.connected = true;
                println!("Connected to {}", address);
                true
            }
            Err(e) => {
                println!("Could not connect to socket: {}", e);
                false
            }
        }
    }

    /// Disconnect from the socket.
    fn disconnect(&mut self) {
        if let Some(stream) = self.stream.take() {
            stream.shutdown(std::net::Shutdown::Both).expect("Shutdown failed");
        }
        self.connected = false;
    }

    /// Enable QMP (QEMU Machine Protocol) capabilities.
    fn enable_commands(&mut self) -> bool {
        if !self.connected {
            return false;
        }

        let message = json!({
            "execute": "qmp_capabilities"
        });

        self.send_message(message.to_string())
    }

    /// Send a button up or down event
    fn send_button(&mut self, button: MouseButton, down: bool) -> Result<()> {
        if !self.connected {
            return Err(InputFlowError::Uninitialized);
        }

        let button_type = match button {
            MouseButton::Left => "left",
            MouseButton::Right => "right",
            MouseButton::Middle => "middle",
            MouseButton::XButton1 => "side",
            MouseButton::XButton2 => "extra",
            _ => return Err(InputFlowError::InvalidKey),
        };

        let message = json!({
            "execute": "input-send-event",
            "arguments": {
                "events": [
                    {
                        "type": "btn",
                        "data": {
                            "button": button_type,
                            "down": down
                        }
                    },
                    // This is for testing if qmp needs a mouse move event to make clicks happen
                    // {
                    //     "type": "rel",
                    //     "data": {
                    //         "axis": "x",
                    //         "value": 0
                    //     }
                    // },
                    // {
                    //     "type": "rel",
                    //     "data": {
                    //         "axis": "y",
                    //         "value": 0
                    //     }
                    // }
                ]
            }
        });

        self.send_message(message.to_string())
            .then(|| Ok(()))
            .unwrap_or_else(|| Err(InputFlowError::SendError))?;
        // This is for testing if qmp needs a mouse move event to make clicks happen
        // self.move_mouse(0, 0);
        Ok(())
    }

    /// Move the mouse by delta_x and delta_y.
    fn move_mouse(&mut self, delta_x: i32, delta_y: i32) -> bool {
        if !self.connected {
            return false;
        }

        let message = json!({
            "execute": "input-send-event",
            "arguments": {
                "events": [
                    {
                        "type": "rel",
                        "data": {
                            "axis": "x",
                            "value": delta_x
                        }
                    },
                    {
                        "type": "rel",
                        "data": {
                            "axis": "y",
                            "value": delta_y
                        }
                    }
                ]
            }
        });

        self.send_message(message.to_string())
    }

    /// Send a message to the connected socket.
    fn send_message(&mut self, message: String) -> bool {
        if let Some(stream) = &mut self.stream {
            if let Err(e) = stream.write_all(message.as_bytes()) {
                println!("Failed to send message: {}", e);
                return false;
            }
            true
        } else {
            println!("No active connection.");
            false
        }
    }
}

// auto disconnect on drop so the user doesn't have to
impl Drop for InputFlowQmp {
    fn drop(&mut self) {
        if self.connected {
            self.disconnect();
        }
    }
}

// Define a `PluginInner` to work with the InputFlow plugin framework.
#[derive(Default)]
struct NativePluginRoot {
    controller: InputFlowQmp,
}

impl<'a> PluginInner<'a> for NativePluginRoot {
    type BorrowedType = Fwd<&'a mut InputFlowQmp>;
    type OwnedType = InputFlowQmp;
    type OwnedTypeMut = InputFlowQmp;

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

// Implement the mouse functionality for the InputFlow plugin.
impl MouseWriter for InputFlowQmp {
    /// Sends a mouse button press down event.
    fn send_button_down(&mut self, button: MouseButton) -> Result<()> {
        if !self.connected {
            return Err(InputFlowError::Uninitialized);
        }

        return self.send_button(button, true);
    }

    /// Releases a mouse button that was set to down previously.
    fn send_button_up(&mut self, button: MouseButton) -> Result<()> {
        if !self.connected {
            return Err(InputFlowError::Uninitialized);
        }

        return self.send_button(button, false);
    }

    /// Presses a mouse button and lets it go all in one for when users do not care about specific timings.
    /// WARNING: this blocks for one milisecond and also is very detectable by anti input automation systems.
    /// It is much more recomended to press and release manually with varied delays
    fn click_button(&mut self, button: MouseButton) -> Result<()> {
        self.send_button_down(button)?;
        std::thread::sleep(std::time::Duration::from_millis(1));
        self.send_button_up(button)
    }

    /// Clears all active pressed mouse buttons. Ensures that mouse writer is set back into a neutral state.
    fn clear_buttons(&mut self) -> Result<()> {
        if !self.connected {
            return Err(InputFlowError::Uninitialized);
        }

        // Send an "up" event for each button (left, right, middle, etc.)
        let buttons = vec![
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::XButton1,
            MouseButton::XButton2,
        ];

        for button in buttons {
            self.send_button_up(button)?;
        }

        Ok(())
    }

    /// Sends a mouse move command to move it by `x` dpi-pixels horizontally, and `y` vertically.
    fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        if !self.connected {
            return Err(InputFlowError::Uninitialized);
        }

        self.move_mouse(x, y);
        Ok(())
    }
}

// Plugin initialization and interface.
cglue_impl_group!(InputFlowQmp, ControllerFeatures, {MouseWriter}, {MouseWriter});

#[allow(improper_ctypes_definitions)]
extern "C" fn create_plugin(lib: &CArc<cglue::trait_group::c_void>, args: *const std::ffi::c_char) -> Result<PluginInnerArcBox<'static>> {
    env_logger::builder()
    // .filter_level(log::LevelFilter::Info)
    .init();

    let args = args::parse_args(args).map_err(|e| {
        log::error!("Invalid parameters were passed to inputflow_qmp: {e:?}.");
        InputFlowError::Parameter
    })?;


    let mut new_plugin = NativePluginRoot::default();
    if !new_plugin.controller.connect(args.address.as_str(), args.port) {
        error!("Failed to connect to qmp at {}:{}", args.address, args.port);
        return Err(InputFlowError::Loading);
    }

    if !new_plugin.controller.enable_commands() {
        return Err(InputFlowError::Loading);
    }
    
    Ok(trait_obj!((new_plugin, lib.clone()) as PluginInner))
}

#[no_mangle]
pub static IF_PLUGIN_HEAD: PluginHeader = PluginHeader {
    features: FeatureSupport::from_bits_retain(
        FeatureSupport::WRITE_MOUSE.bits(),
    ),
    layout: ROOT_LAYOUT,
    create: create_plugin,
};
