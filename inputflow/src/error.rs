use cglue::result::IntError;
use std::num::NonZeroI32;

/// Describes possible errors that can occur loading the library
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InputFlowError {
    Path = 1,
    Loading = 2,
    Symbol = 3,
    Abi = 4,
    InvalidKey = 5,
    SendError = 6,
}

impl IntError for InputFlowError {
    fn into_int_err(self) -> NonZeroI32 {
        NonZeroI32::new(self as u8 as _).unwrap()
    }

    fn from_int_err(err: NonZeroI32) -> Self {
        match err.get() {
            1 => Self::Path,
            2 => Self::Loading,
            3 => Self::Symbol,
            4 => Self::Abi,
            5 => Self::InvalidKey,
            6 => Self::SendError,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for InputFlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InputFlowError {}

pub type Result<T> = std::result::Result<T, InputFlowError>;
