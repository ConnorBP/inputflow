//! The user of the plugin API.
//!
//! This crate loads binary plugins using the API, and performs some operations with mandatory and
//! optional traits.

use inputflow::{
    cglue::{result::from_int_result, *},
    error::InputFlowError,
    prelude::*,
    *,
};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::{error, io};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut lib = String::new();

    println!("Enter name of the plugin library [inputflow_native]:");

    io::stdin().read_line(&mut lib)?;

    if lib.trim().is_empty() {
        lib = "inputflow_native".to_string();
    }

    let mut obj = MaybeUninit::uninit();
    let res = unsafe {
        load_plugin(
            CString::new(lib.trim()).unwrap().as_c_str().into(),
            &mut obj,
        )
    };
    let mut obj = unsafe { from_int_result::<_, InputFlowError>(res, obj) }?;

    {
        let mut borrowed = obj.borrow_features();

        println!("loaded {} with features: {:#b}", borrowed.name(), borrowed.capabilities());

        if let Some(obj) = as_mut!(borrowed impl MouseWriter) {
            println!("Using borrowed mouse:");
            // use_kvstore(obj)?;
            obj.send_button_down(1)?;
            obj.send_button_up(1)?;
        }

        if let Some(obj) = as_mut!(borrowed impl MouseWriter) {
            println!("clearing buttons:");
            obj.clear_buttons()?;
        }

        println!("Borrowed done.");
    }

    {
        let mut owned = obj.into_features();

        if let Some(obj) = as_mut!(owned impl MouseWriter) {
            println!("Using owned MouseWriter:");
            for i in 0..100 {
                obj.mouse_move_relative(1, 0)?
            }
        }

        // Casting can be combined with a multiple of optional traits.
        // if let Some(mut obj) = cast!(owned impl MouseWriter + KeyboardWriter) {
        //     println!("Dumping owned kvstore:");
        //     kvdump(&mut obj);

        //     // You can still use the mandatory traits.
        //     obj.print_self();
        // }

        println!("Owned done.");
    }

    println!("Quitting");

    Ok(())
}