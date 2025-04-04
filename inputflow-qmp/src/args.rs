// Arguments for the plugin initialization

use ::std::error::Error;

// default config values for each platform

/// Returns the default value for ip address
fn default_ip() -> String {
    "127.0.0.1".to_string()
}

/// Returns the default port expected for qmp to be opened at
fn default_port() -> u32 {
    6448
}

#[derive(::serde::Serialize, ::serde::Deserialize)]
pub(crate) struct Args {
    /// The ip address to connect to for qmp (defaults to 127.0.0.1)
    #[serde(default = "default_ip")]
    pub address: String,
    /// The qmp port as exposed by qemu
    #[serde(default = "default_port")]
    pub port: u32,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 6448
        }
    }
}

/// handles the raw string args to extract useful information for initialization
pub(super) fn parse_args(args: *const std::ffi::c_char)  -> Result<Args, Box<dyn Error>> {
    // safety:
    // the plugin caller created this string with into_raw
    // therefor we must use from_raw on it to make sure it dealocates afterwards
    // furthermore, we will not be modifying the string or its length
    let args_str = unsafe{std::ffi::CString::from_raw(args as *mut _)};

    if args_str.is_empty() {
        Ok(Args::default())
    } else {
        log::info!("inputflow_qmp received args: {args_str:?}");
        Ok(ron::from_str(args_str.to_str()?)?)
    }
}