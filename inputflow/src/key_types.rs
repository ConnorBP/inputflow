//! Standardized keycode interface across all plugins.
//! Based on the Microsoft Virtual Keycode standard with some minor variation.

use abi_stable::StableAbi;

/// Mouse Buttons are a simple byte enum that gets passed around
/// Values mostly align with the microsoft VKEY spec
#[repr(u8)]
#[derive(Debug, Clone, Copy, StableAbi)]
pub enum MouseButton {
    NULL,
    Left,
    Right,
    UNUSED01, // this is VK_CANCEL usually
    Middle,
    XButton1,
    XButton2,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}

impl std::fmt::Display for MouseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Keyboard keys are also represented by a byte
/// Values mostly align with the microsoft VKEY spec
#[repr(u8)]
#[derive(Debug, Clone, Copy, StableAbi)]
pub enum KeyboardKey {
    NULL,
    A = 0x41, // ascii code for 'A'. Same as VKEY
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LWIN,
    RWIN,
    APPS,
    _Reserved,
    SLEEP,
    NUMPAD0,
    NUMPAD1,
    NUMPAD2,
    NUMPAD3,
    NUMPAD4,
    NUMPAD5,
    NUMPAD6,
    NUMPAD7,
    NUMPAD8,
    NUMPAD9,
}

impl std::fmt::Display for KeyboardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
