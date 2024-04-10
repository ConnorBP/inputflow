//! Example plugin library.
//!
//! This plugin crate will not be known to the user, both parties will interact with the help of
//! the shared plugin API.

use enigo::{Button, Direction, Enigo, Keyboard, Mouse, Settings};
use inputflow::prelude::*;

#[derive(Default)]
struct NativePluginRoot {
    controller: InputFlowNative,
}

impl<'a> PluginInner<'a> for NativePluginRoot {
    type BorrowedType = Fwd<&'a mut InputFlowNative>;

    type OwnedType = InputFlowNative;
    type OwnedTypeMut = InputFlowNative;

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
pub struct InputFlowNative {
    enigo: Enigo,
}

impl Clone for InputFlowNative {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl Loadable for InputFlowNative {
    fn name(&self) -> abi_stable::std_types::RString {
        "inputflow_native".into()
    }

    fn capabilities(&self) -> u8 {
        IF_PLUGIN_HEAD.features.bits()
    }
}

impl Default for InputFlowNative {
    fn default() -> Self {
        Self {
            enigo: Enigo::new(&Settings {
                release_keys_when_dropped: true,
                ..Default::default()
            })
            .expect("setting up enigo"),
        }
    }
}

/// VK_LBUTTON	0x01	Left mouse button
/// VK_RBUTTON	0x02	Right mouse button
/// VK_CANCEL	0x03	Control-break processing
/// VK_MBUTTON	0x04	Middle mouse button
/// VK_XBUTTON1	0x05	X1 mouse button
/// VK_XBUTTON2	0x06	X2 mouse button
/// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
/// Buttons align with windows VKEY spec, scroll is appended
/// TODO: move this to the library crate so the interface is standadized for all plugins
fn keycode_to_button(btn: u32) -> Option<Button> {
    match btn {
        0x01 => Some(Button::Left),
        0x02 => Some(Button::Right),
        0x03 => None, // Reserved in vkey spec. Maybe we will use it for something?
        0x04 => Some(Button::Middle),
        0x05 => Some(Button::Back),
        0x06 => Some(Button::Forward),
        0x07 => Some(Button::ScrollUp),
        0x08 => Some(Button::ScrollDown),
        0x09 => Some(Button::ScrollLeft),
        0x10 => Some(Button::ScrollRight),
        _ => None,
    }
}

impl KeyboardWriter for InputFlowNative {
    #[doc = r" Sends keyboard press down event"]
    fn send_key_down(&mut self, key: u32) -> Result<()> {
        todo!()
    }

    #[doc = r" Releases a key that was set to down previously"]
    fn send_key_up(&mut self, key: u32) -> Result<()> {
        todo!()
    }

    #[doc = r" Presses a key and lets it go all in one for when users do not care about specific timings"]
    fn press_key(&mut self, key: u32) -> Result<()> {
        todo!()
    }

    #[doc = r" clears all active pressed keys. Useful for cleaning up multiple keys presses in one go."]
    #[doc = r" Ensures that keyboard writer is set back into a neutral state."]
    fn clear_keys(&mut self) -> Result<()> {
        todo!()
    }
}

impl MouseWriter for InputFlowNative {
    #[doc = r" Sends mouse button press down event"]
    fn send_button_down(&mut self, button: u32) -> Result<()> {
        if let Some(key) = keycode_to_button(button) {
            self.enigo
                .button(key, Direction::Press)
                .map_err(|_| InputFlowError::SendError)
        } else {
            Err(InputFlowError::InvalidKey)
        }
    }

    #[doc = r" Releases a mouse button that was set to down previously"]
    fn send_button_up(&mut self, button: u32) -> Result<()> {
        if let Some(button) = keycode_to_button(button) {
            self.enigo
                .button(button, Direction::Release)
                .map_err(|_| InputFlowError::SendError)
        } else {
            Err(InputFlowError::InvalidKey)
        }
    }

    #[doc = r" Presses a  mouse button and lets it go all in one for when users do not care about specific timings"]
    fn click_button(&mut self, button: u32) -> Result<()> {
        if let Some(button) = keycode_to_button(button) {
            self.enigo
                .button(button, Direction::Click)
                .map_err(|_| InputFlowError::SendError)
        } else {
            Err(InputFlowError::InvalidKey)
        }
    }

    #[doc = r" clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go."]
    #[doc = r" Ensures that mouse writer is set back into a neutral state."]
    fn clear_buttons(&mut self) -> Result<()> {
        let (held_keys, held_keycodes) = self.enigo.held();
        for key in held_keys {
            if self.enigo.key(key, Direction::Release).is_err() {
                println!("WARN: unable to release {key:?}");
            };
        }
        for keycode in held_keycodes {
            if self.enigo.raw(keycode, Direction::Release).is_err() {
                println!("WARN: unable to release {keycode:?}");
            };
        }
        Ok(())
    }

    #[doc = r" Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically"]
    fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo
            .move_mouse(x, y, enigo::Coordinate::Rel)
            .map_err(|_| InputFlowError::SendError)
    }
}

cglue_impl_group!(InputFlowNative, ControllerFeatures,{KeyboardWriter, MouseWriter}, {KeyboardWriter, MouseWriter} );

extern "C" fn create_plugin(lib: &CArc<cglue::trait_group::c_void>) -> PluginInnerArcBox<'static> {
    // type_identity!();
    trait_obj!((NativePluginRoot::default(), lib.clone()) as PluginInner)
}

#[no_mangle]
pub static IF_PLUGIN_HEAD: PluginHeader = PluginHeader {
    features: FeatureSupport::from_bits_retain(
        FeatureSupport::WRITE_KEYBOARD.bits() | FeatureSupport::WRITE_MOUSE.bits(),
    ),
    layout: ROOT_LAYOUT,
    create: create_plugin,
};
