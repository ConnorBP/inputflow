//! Example plugin library.
//!
//! This plugin crate will not be known to the user, both parties will interact with the help of
//! the shared plugin API.

use inputflow::prelude::*;
use inputflow::*;
use inputflow::cglue;
use inputflow::cglue::*;
use v1::abi_stable::type_identity;

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

#[derive(Debug, Default, Clone)]
pub struct InputFlowNative {}

impl Loadable for InputFlowNative {
    fn name(&self) -> abi_stable::std_types::RString {
        "inputflow_win32_native".into()
    }

    fn capabilities(&self) -> u8 {
        IF_PLUGIN_HEAD.features.bits()
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
    fn init(&mut self) {
        todo!()
    }

    #[doc = r" Sends mouse button press down event"]
    fn send_key_down(&mut self, key: u32) -> Result<()> {
        todo!()
    }

    #[doc = r" Releases a mouse button that was set to down previously"]
    fn send_key_up(&mut self, key: u32) -> Result<()> {
        todo!()
    }

    #[doc = r" Presses a  mouse button and lets it go all in one for when users do not care about specific timings"]
    fn press_key(&mut self, key: u32) -> Result<()> {
        todo!()
    }

    #[doc = r" clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go."]
    #[doc = r" Ensures that mouse writer is set back into a neutral state."]
    fn clear_keys(&mut self) -> Result<()> {
        todo!()
    }

    #[doc = r" Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically"]
    fn mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        todo!()
    }
}

cglue_impl_group!(InputFlowNative, ControllerFeatures,{KeyboardWriter, MouseWriter}, {KeyboardWriter, MouseWriter} );

extern "C" fn create_plugin(lib: &CArc<cglue::trait_group::c_void>)-> PluginInnerArcBox<'static> {
    // type_identity!();
    trait_obj!((NativePluginRoot::default(), lib.clone()) as PluginInner)
}


#[no_mangle]
pub static IF_PLUGIN_HEAD: PluginHeader = PluginHeader {
    features: FeatureSupport::from_bits_retain(FeatureSupport::WRITE_KEYBOARD.bits() | FeatureSupport::WRITE_MOUSE.bits()),
    layout: ROOT_LAYOUT,
    create: create_plugin,
};
