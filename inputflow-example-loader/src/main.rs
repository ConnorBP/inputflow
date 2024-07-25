//! The user of the plugin API.
//!
//! This crate loads binary plugins using the API, and performs some operations with mandatory and
//! optional traits.

use ::std::time::Duration;
use inputflow::{
    cglue::{result::from_int_result, *},
    error::InputFlowError,
    prelude::*,
};
use std::ffi::CString;
use std::io;
use std::mem::MaybeUninit;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut lib = String::new();

    println!("Enter name of the plugin library [inputflow_native]:");

    io::stdin().read_line(&mut lib)?;

    if lib.trim().is_empty() {
        lib = "inputflow_native".to_string();
    }

    println!("Enter plugin args:");
    let mut args = String::new();
    io::stdin().read_line(&mut args)?;

    let mut obj = MaybeUninit::uninit();
    let res = unsafe {
        load_plugin(
            CString::new(lib.trim()).unwrap().as_c_str().into(),
            CString::new(args.trim()).unwrap().as_c_str().into(),
            &mut obj,
        )
    };
    let mut obj = unsafe { from_int_result::<_, InputFlowError>(res, obj) }?;

    {
        let mut borrowed = obj.borrow_features();

        if let Some(features) = FeatureSupport::from_bits(borrowed.capabilities()) {
            println!("loaded {} with features: {:?}", borrowed.name(), features);

            // borrow a generic trait object of type &mut (impl Loadable + MouseWriter)
            if let Some(obj) = as_mut!(borrowed impl MouseWriter) {
                println!("Using borrowed mouse:");
                obj.send_button_down(MouseButton::Left)?;
                obj.send_button_up(MouseButton::Left)?;
            }

            if let Some(obj) = as_mut!(borrowed impl MouseWriter) {
                println!("clearing buttons:");
                obj.clear_buttons()?;
            }
        } else {
            println!(
                "ERROR: Some features were not valid in bytes: {:#b}",
                borrowed.capabilities()
            );
        }

        println!("Borrowed done.");
    }

    {
        let mut owned = obj.into_features();

        if let Some(obj) = as_mut!(owned impl MouseWriter) {
            println!("Using owned MouseWriter:");
            let scale = 5;
            // wigg the mouse out for a few seconds
            for i in 0..1000 {
                let x = (i % (5 * scale)) - 2 * scale;
                let y = (i - 2) % (7 * scale) - 3 * scale;
                obj.mouse_move_relative(x, y)?;
                std::thread::sleep(Duration::from_millis(4));
            }
        }

        // Casting can be combined with a multiple of optional traits.
        if let Some(mut obj) = cast!(owned impl MouseWriter + KeyboardWriter) {
            println!("Clearing keyboard keys");
            obj.clear_keys()?;
            // You can still use the mandatory traits.
            obj.name();
        }

        println!("Owned done.");
    }

    println!("Quitting");

    Ok(())
}
