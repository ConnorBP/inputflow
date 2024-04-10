pub mod api_traits;
pub mod error;
pub mod headers;

use abi_stable::type_layout::TypeLayout;
use abi_stable::StableAbi;
use api_traits::{ControllerFeatures, Loadable};
use cglue::prelude::v1::{trait_group::compare_layouts, *};
use core::mem::MaybeUninit;
use error::{InputFlowError, Result};
use headers::PluginHeader;
use libloading::{library_filename, Library, Symbol};

#[cglue_trait]
pub trait PluginInner<'a> {
    #[wrap_with_group(ControllerFeatures)]
    type BorrowedType: Loadable + 'a;
    #[wrap_with_group(ControllerFeatures)]
    type OwnedType: Loadable + 'static;
    #[wrap_with_group_mut(ControllerFeatures)]
    type OwnedTypeMut: Loadable + 'a;

    fn borrow_features(&'a mut self) -> Self::BorrowedType;

    fn into_features(self) -> Self::OwnedType;

    fn mut_features(&'a mut self) -> &'a mut Self::OwnedTypeMut;
}

/// Having the inner type with a lifetime allows to borrow features for any lifetime.
///
/// This could be avoided with [GAT](https://rust-lang.github.io/rfcs/1598-generic_associated_types.html)
pub trait Plugin: for<'a> PluginInner<'a> {}
impl<T: for<'a> PluginInner<'a>> Plugin for T {}

// pub type KeyValueCallback<'a> = OpaqueCallback<'a, KeyValue<'a>>;

/// Load a plugin from a given library.
///
/// # Safety
///
/// Input library must implement a correct `create_plugin` and `get_root_layout()` functions.
/// Its signatures must be as follows:
///
/// `extern "C" fn crate_plugin(&CArc<T>) -> PluginInnerArcBox<'static>`
/// `extern "C" fn get_root_layout() -> Option<&'static TypeLayout>`
///
/// Where `T` is any type, since it's opaque. Meanwhile, `get_root_layout` should simply
/// [call the one in this crate](self::get_root_layout). It is used to verify
/// version mismatches.
#[no_mangle]
pub unsafe extern "C" fn load_plugin(
    name: ReprCStr<'_>,
    ok_out: &mut MaybeUninit<PluginInnerArcBox<'static>>,
) -> i32 {
    load_plugin_impl(name.as_ref()).into_int_out_result(ok_out)
}

unsafe fn load_plugin_impl(name: &str) -> Result<PluginInnerArcBox<'static>> {
    let mut current_exe = std::env::current_exe().map_err(|_| InputFlowError::Path)?;
    current_exe.set_file_name(library_filename(name));
    let lib = Library::new(current_exe).map_err(|e| {
        println!("{}", e);
        InputFlowError::Loading
    })?;

    let header: Symbol<&'static PluginHeader> = lib.get(b"IF_PLUGIN_HEAD\0").map_err(|e| {
        println!("{}", e);
        InputFlowError::Symbol
    })?;
    let header = header.into_raw();

    if !compare_layouts(Some(ROOT_LAYOUT), Some(header.layout)).is_valid_strict() {
        return Err(InputFlowError::Abi);
    }

    let arc = CArc::from(lib);
    Ok((header.create)(&arc.into_opaque()))
}

/// Layout for the root vtable.
///
/// Layout that should be embedded to a `PluginHeader`.
/// Other layouts are not necessary, because the very root depends on them already.
#[no_mangle]
pub static ROOT_LAYOUT: &TypeLayout = PluginInnerArcBox::LAYOUT;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[doc(hidden)]
pub mod cglue {
    pub use ::cglue::prelude::v1::*;
}

#[doc(hidden)]
pub mod abi_stable {
    pub use abi_stable::*;
}

#[doc(hidden)]
#[allow(ambiguous_glob_reexports)]
pub mod prelude {
    pub mod v1 {
        pub use crate::abi_stable;
        pub use crate::api_traits::*;
        pub use crate::cglue::*;
        pub use crate::error::*;
        pub use crate::headers::*;
        pub use crate::iter::*;
        pub use crate::*;
    }
    pub use v1::*;
}
