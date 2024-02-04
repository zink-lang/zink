extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use syn::ItemType;

static IOTA: AtomicI32 = AtomicI32::new(0);

/// Parse storage attribute.
///
/// Method `get` unwraps the ptr as the original type, mainly
/// mainly for passing the compilation checks at the moment,
/// and it works for WASM in real cases as well.
///
/// For the cases in EVM, it doesn't matter it returns pointer
/// since the value will be left on stack anyway.
pub fn parse(input: ItemType) -> TokenStream {
    let name = input.ident;
    let ty = input.ty.to_token_stream();

    // Temporary solution, we'll switch to 32 byte storage keys later
    let key = IOTA.fetch_add(1, Relaxed);
    let expanded = quote! {
        #[doc = concat!(" Storage ", stringify!($variable_name))]
        struct #name;

        impl zink::Storage<#ty> for #name {
            const STORAGE_KEY: i32 = #key;


            fn get() -> #ty {
                zink::Asm::push(Self::STORAGE_KEY);
                unsafe {
                    *(zink::ffi::evm::sload() as *const #ty)
                }
            }

            fn set(value: #ty) {
                zink::Asm::push(value);
                zink::Asm::push(Self::STORAGE_KEY);
                unsafe {
                    zink::ffi::evm::sstore();
                }
            }
        }
    };

    expanded
}
