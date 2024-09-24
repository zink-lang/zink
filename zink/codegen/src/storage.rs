extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use std::{cell::RefCell, collections::HashSet};
use syn::{Ident, ItemStruct};

thread_local! {
   static STORAGE_REGISTRY: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Parse storage attribute.
///
/// Method `get` unwraps the ptr as the original type, mainly
/// mainly for passing the compilation checks at the moment,
/// and it works for WASM in real cases as well.
///
/// For the cases in EVM, it doesn't matter it returns pointer
/// since the value will be left on stack anyway.
pub fn parse(attr: TokenStream, input: ItemStruct) -> TokenStream {
    let name = input.ident;
    // let ty = input.ty.to_token_stream();

    storage_kv(name, attr)
    // match ty.to_string() {
    //     // m if m.starts_with("Mapping") => storage_mapping(name, ty),
    //     _ => storage_kv(name, attr),
    // }
}

#[allow(unused)]
/// Expand storage mapping
pub fn storage_mapping(name: Ident, _ty: TokenStream) -> TokenStream {
    let _key = storage_index(name.to_string());
    let expanded = quote! {
        #[doc = concat!(" Storage ", stringify!($variable_name))]
        struct #name;

        // impl zink::Storage for #name {
        //     const STORAGE_KEY: i32 = #key;
        // }
    };

    expanded
}

fn storage_kv(name: Ident, ty: TokenStream) -> TokenStream {
    // Temporary solution, we'll switch to 32 byte storage keys later
    let key = storage_index(name.to_string());
    let expanded = quote! {
        #[doc = concat!(" Storage ", stringify!($variable_name))]
        struct #name;

        impl zink::storage::Storage for #name {
            type Value = #ty;
            const STORAGE_KEY: i32 = #key;
        }
    };

    expanded
}

fn storage_index(name: String) -> i32 {
    STORAGE_REGISTRY.with_borrow_mut(|r| {
        let key = r.len();
        if !r.insert(name.clone()) {
            panic!("Storage {name} has already been declared");
        }

        key
    }) as i32
}
