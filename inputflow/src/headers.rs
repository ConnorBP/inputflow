use abi_stable::type_layout::TypeLayout;
use cglue::prelude::v1::*;

use crate::{error::Result, PluginInnerArcBox};

bitflags::bitflags! {
    /// Defines what features this plugin supports
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(C)]
    pub struct FeatureSupport: u8 {
        const READ_MOUSE = 0x01;
        const WRITE_MOUSE = 0x02;
        const READ_KEYBOARD = 0x04;
        const WRITE_KEYBOARD = 0x08;
        const INTERCEPT_MOUSE = 0x10;
        const INTERCEPT_KEYBOARD = 0x20;
        const ALL = Self::READ_MOUSE.bits()
                    | Self::WRITE_MOUSE.bits()
                    | Self::READ_KEYBOARD.bits()
                    | Self::WRITE_KEYBOARD.bits()
                    | Self::INTERCEPT_MOUSE.bits()
                    | Self::INTERCEPT_KEYBOARD.bits();
    }
}

// impl StableAbi for FeatureSupport {
//     type IsNonZeroType = True;

//     const LAYOUT: &'static TypeLayout = TypeLayout::ABI_CONSTS;
// }

/// Plugin header that the API looks for.
///
/// Plugins should define the header with name `PLUGIN_HEADER` with no mangling.
/// the function interface exposed by plugins, "create", accepts a reference to the library interface
/// as well as a raw ffi c_str (char ptr) to the arguments for plugin init
/// each plugin may decide how to parse these arguments themselves and must provide documentation on their usage.
#[repr(C)]
pub struct PluginHeader {
    pub features: FeatureSupport,
    pub layout: &'static TypeLayout,
    pub create: extern "C" fn(&CArc<cglue::trait_group::c_void>, *const std::ffi::c_char) -> Result<PluginInnerArcBox<'static>>,
}
