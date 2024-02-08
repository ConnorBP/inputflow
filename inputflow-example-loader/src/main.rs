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

    // {
    //     let mut borrowed = obj.borrow_features();

    //     borrowed.print_self();

    //     if let Some(obj) = as_mut!(borrowed impl KeyValueStore) {
    //         println!("Using borrowed kvstore:");
    //         use_kvstore(obj)?;
    //     }

    //     if let Some(obj) = as_mut!(borrowed impl KeyValueDumper) {
    //         println!("Dumping borrowed kvstore:");
    //         kvdump(obj);
    //     }

    //     println!("Borrowed done.");
    // }

    // {
    //     let mut owned = obj.into_features();

    //     owned.print_self();

    //     if let Some(obj) = as_mut!(owned impl KeyValueStore) {
    //         println!("Using owned kvstore:");
    //         use_kvstore(obj)?;
    //     }

    //     // Casting can be combined with a multiple of optional traits.
    //     if let Some(mut obj) = cast!(owned impl KeyValueDumper + KeyValueStore) {
    //         println!("Dumping owned kvstore:");
    //         kvdump(&mut obj);

    //         // You can still use the mandatory traits.
    //         obj.print_self();
    //     }

    //     println!("Owned done.");
    // }

    println!("Quitting");

    Ok(())
}