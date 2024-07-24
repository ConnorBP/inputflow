// Arguments for the plugin initialization

use ::std::error::Error;

#[derive(::serde::Serialize, ::serde::Deserialize)]
pub(crate) struct Args {
    pub com_port: String,
    pub baud_rate: u32,
    pub timeout_ms: u64,
}

/// handles the raw string args to extract useful information for initialization
pub(super) fn parse_args(args: *const std::ffi::c_char)  -> Result<Args, Box<dyn Error>> {
    // safety:
    // the plugin caller created this string with into_raw
    // therefor we must use from_raw on it to make sure it dealocates afterwards
    // furthermore, we will not be modifying the string or its length
    Ok(ron::from_str(unsafe{std::ffi::CString::from_raw(args as *mut _)}.to_str()?)?)
}