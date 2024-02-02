pub use abi_stable::type_layout::TypeLayout;
use abi_stable::StableAbi;
use cglue::prelude::v1::{trait_group::compare_layouts, *};
use core::mem::MaybeUninit;
use core::num::NonZeroI32;
use libloading::{library_filename, Library, Symbol};

use crate::error::Result;

/// Main interface for loadable plugins
pub trait Loadable {
    type Instance: StableAbi;
    type InputArg;
    type CInputArg: StableAbi;
    type ArgsType;
}

/// Provides ability to send keyboard input to a device (local or external).
#[cfg_attr(feature = "plugins", cglue_trait)]
#[int_result]
pub trait KeyboardWriter: Send {
    /// Sends keyboard press down event
    fn send_key_down(&mut self, key: u32) -> Result<()>;

    /// Releases a key that was set to down previously
    fn send_key_up(&mut self, key: u32) -> Result<()>;

    /// Presses a key and lets it go all in one for when users do not care about specific timings
    fn press_key(&mut self, key: u32) -> Result<()>;

    /// clears all active pressed keys. Useful for cleaning up multiple keys presses in one go.
    /// Ensures that keyboard writer is set back into a neutral state.
    fn clear_keys(&mut self) -> Result<()>;
}

// TODO Later on. For now, memflow handles this.
// #[cfg_attr(feature = "plugins", cglue_trait)]
// pub trait KeyboardReader {
//
// }


/// Provides ability to send mouse button input to a device (local or external).
/// Also allows mouse movement input
#[cfg_attr(feature = "plugins", cglue_trait)]
#[int_result]
pub trait MouseWriter: Send {

    fn init(&mut self);

    /// Sends mouse button press down event
    fn send_key_down(&mut self, key: u32) -> Result<()>;

    /// Releases a mouse button that was set to down previously
    fn send_key_up(&mut self, key: u32) -> Result<()>;

    /// Presses a  mouse button and lets it go all in one for when users do not care about specific timings
    fn press_key(&mut self, key: u32) -> Result<()>;

    /// clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go.
    /// Ensures that mouse writer is set back into a neutral state.
    fn clear_keys(&mut self) -> Result<()>;


    // mouse move abilities (might make this a separate trait. Undecided):

    /// Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically
    fn mouse_move(&mut self, x: i32, y: i32) -> Result<()>;

}

// TODO Later on. For now, memflow handles this.
// #[cfg_attr(feature = "plugins", cglue_trait)]
// pub trait MouseReader {
//
// }

/// Defines what features this plugin supports
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct FeatureSupport: u8 {
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