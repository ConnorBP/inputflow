// Arguments for the plugin initialization

use ::std::error::Error;

// default config values for each platform

/// Returns the default value for serial port name or path
/// on windows this is generally `COM#`
/// on unix this is generally a path like `/dev/ttyUSB#`
fn default_com_port() -> String {
    #[cfg(target_family="unix")]
    {"/dev/ttyUSB0".to_string()}
    #[cfg(target_family="windows")]
    {"COM6".to_string()}
}

/// Returns the default serial device name to look for
/// this is used when auto port find mode is enabled instead
/// specifying of manual com path
fn default_device_name() -> String {
    "USB-SERIAL CH340".to_string()
}

/// Auto port selection by device name is the default behaviour
fn default_auto_select() -> bool {
    true
}

/// Default serial baud rate is the default of KMBox:
/// one hundred and fifteen thousand, two hundred.
fn default_baud_rate() -> u32 {
    115_200
}

#[derive(::serde::Serialize, ::serde::Deserialize)]
pub(crate) struct Args {
    /// Automatically selects the serial port
    /// based on the device name argument if true
    #[serde(default = "default_auto_select")]
    pub auto_select: bool,
    /// The device name to search for in auto mode.
    /// defaults to `USB-SERIAL CH340`.
    #[serde(default = "default_device_name")]
    pub device_name: String,
    /// The serial port path to connect to (used when auto_select is false)
    /// defaults to `COM6` on Windows and `/dev/ttyUSB0` on Unix
    #[serde(default = "default_com_port")]
    pub com_port: String,
    #[serde(default = "default_baud_rate")]
    pub baud_rate: u32,
    /// Port timeout value in milliseconds.
    /// Defaults to zero (no timeout)
    #[serde(default)]
    pub timeout_ms: u64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            auto_select: true,
            device_name: "USB-SERIAL CH340".to_string(),
            com_port: "COM6".to_string(),
            baud_rate: 115200,
            timeout_ms: Default::default() }
    }
}

/// handles the raw string args to extract useful information for initialization
pub(super) fn parse_args(args: *const std::ffi::c_char)  -> Result<Args, Box<dyn Error>> {
    // safety:
    // the plugin caller created this string with into_raw
    // therefor we must use from_raw on it to make sure it dealocates afterwards
    // furthermore, we will not be modifying the string or its length
    let args_str = unsafe{std::ffi::CString::from_raw(args as *mut _)};

    if(args_str.is_empty()) {
        Ok(Args::default())
    } else {
        log::info!("inputflow_kmbox received args: {args_str:?}");
        Ok(ron::from_str(args_str.to_str()?)?)
    }
}