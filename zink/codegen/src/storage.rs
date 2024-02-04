extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use syn::ItemType;

static IOTA: AtomicI32 = AtomicI32::new(0);

/// Parse storage attribute.
///
/// Method `get` unwraps the ptr as the original type in WASM,
/// mainly for passing the compilation checks at the moment,
/// it works for WASM as well.
///
/// But for the cases in EVM, it doesn't matter what if it returns
/// pointer since the value will be left on stack anyway.
pub fn parse(input: ItemType) -> TokenStream {
    let name = input.ident;
    let ty = input.ty.to_token_stream();

    match ty.to_string().as_str() {
        "i32" => (),
        _ => unimplemented!("Only support i32 as storage key for now."),
    };

    // temporary solution, we'll switch to 32 byte storage keys later
    let key = IOTA.fetch_add(1, Relaxed);
    let expanded = quote! {
        #[doc = concat!(" Storage ", stringify!($variable_name))]
        struct #name;

        impl zink::Storage<#ty> for #name {
            const STORAGE_KEY: i32 = #key;

            fn get() -> #ty {
                unsafe {
                    *(zink::ffi::evm::sload(Self::STORAGE_KEY) as *const #ty)
                }
            }

            fn set(value: #ty) {
                unsafe {
                    zink::Asm::push(&value);
                    zink::ffi::evm::sstore(Self::STORAGE_KEY);
                }
            }
        }
    };

    expanded
}
