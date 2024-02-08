pub use abi_stable::type_layout::TypeLayout;
use cglue::prelude::v1::*;
use crate::error::Result;


#[cfg_attr(feature = "plugins", cglue_trait, cglue_forward)]//
/// Main interface for loadable plugins
pub trait Loadable {
    
    // type Instance: StableAbi;
    // type InputArg;
    // type CInputArg: StableAbi;
    // type ArgsType;

    fn name(&self) -> abi_stable::std_types::RString;
    /// U8 bitflags of capabilities. Sadly I have not yet figured out how to get the bitflags crate to play nice with abi_stable so this is base type for now
    fn capabilities(&self) -> u8;
}

cglue_trait_group!(ControllerFeatures, { Loadable }, { KeyboardWriter, MouseWriter, Clone });

/// Provides ability to send keyboard input to a device (local or external).
#[cfg_attr(feature = "plugins", cglue_trait, cglue_forward)]
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
#[cfg_attr(feature = "plugins", cglue_trait, cglue_forward)]
#[int_result]
pub trait MouseWriter: Send {
    /// Sends mouse button press down event
    fn send_button_down(&mut self, key: u32) -> Result<()>;

    /// Releases a mouse button that was set to down previously
    fn send_button_up(&mut self, key: u32) -> Result<()>;

    /// Presses a  mouse button and lets it go all in one for when users do not care about specific timings
    fn click_button(&mut self, key: u32) -> Result<()>;

    /// clears all active pressed  mouse buttons. Useful for cleaning up multiple mouse button presses in one go.
    /// Ensures that mouse writer is set back into a neutral state.
    fn clear_buttons(&mut self) -> Result<()>;

    // mouse move abilities (might make this a separate trait. Undecided):

    /// Sends a mouse move command to move it x dpi-pixels horizontally, and y vertically
    fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()>;
    
}

// TODO Later on. For now, memflow handles this.
// #[cfg_attr(feature = "plugins", cglue_trait)]
// pub trait MouseReader {
//
// }
