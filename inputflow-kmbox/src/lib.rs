//! KMBox plugin for inputflow.
//! Controls user input over serial interface to KMBox device.

use ::std::time::Duration;

use inputflow::prelude::*;
use serialport::SerialPort;

mod args;

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
    port: Box<dyn SerialPort>,
}

// impl Clone for InputFlowKMBox {
//     fn clone(&self) -> Self {
//         Self::default()
//     }
// }

impl Loadable for InputFlowKMBox {
    fn name(&self) -> abi_stable::std_types::RString {
        "inputflow_kmbox".into()
    }

    fn capabilities(&self) -> u8 {
        IF_PLUGIN_HEAD.features.bits()
    }
}

// impl Default for InputFlowKMBox {
//     fn default() -> Self {
//         Self {
            
//         }
//     }
// }


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

fn write_move(x: i32, y: i32) {

}

impl MouseWriter for InputFlowKMBox {
    #[doc = r" Sends mouse button press down event"]
    fn send_button_down(&mut self, button: MouseButton) -> Result<()> {
        todo!();
    }

    #[doc = r" Releases a mouse button that was set to down previously"]
    fn send_button_up(&mut self, button: MouseButton) -> Result<()> {
        todo!();
    }

    #[doc = r" Presses a  mouse button and lets it go all in one for when users do not care about specific timings"]
    fn click_button(&mut self, button: MouseButton) -> Result<()> {
        todo!();
    }

    #[doc = r" clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go."]
    #[doc = r" Ensures that mouse writer is set back into a neutral state."]
    fn clear_buttons(&mut self) -> Result<()> {
        todo!();
    }

    #[doc = r" Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically"]
    fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        todo!();
    }
}

cglue_impl_group!(InputFlowKMBox, ControllerFeatures,{KeyboardWriter, MouseWriter}, {KeyboardWriter, MouseWriter} );

extern "C" fn create_plugin(lib: &CArc<cglue::trait_group::c_void>, args: *const std::ffi::c_char) -> PluginInnerArcBox<'static> {
    trait_obj!(
        (
            KMBoxPluginRoot::new(
                args::parse_args(args).expect("parsing args")
            ).expect("initializing plugin"),
            lib.clone()
        ) as PluginInner
    )
}

#[no_mangle]
pub static IF_PLUGIN_HEAD: PluginHeader = PluginHeader {
    features: FeatureSupport::from_bits_retain(
        FeatureSupport::WRITE_KEYBOARD.bits() | FeatureSupport::WRITE_MOUSE.bits(),
    ),
    layout: ROOT_LAYOUT,
    create: create_plugin,
};
