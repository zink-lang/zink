extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use syn::ItemType;

pub fn parse(input: ItemType) -> TokenStream {
    let variable_name = input.ident;
    let variable_type = input.ty.to_token_stream();

    match variable_type.to_string().as_str() {
        "i32" => (),
        _ => unimplemented!("Only support i32 as storage key for now."),
    };

    // hash-based storage key derivation (we decided that order-based is better)

    // let mut h = Keccak256::new();
    // h.update(variable_name.to_string().as_bytes());
    // let storage_key = h.finalize();
    //
    // // lmfao i'm sure there's a better way to do this but i don't know how
    // let mut storage_key_string = storage_key.as_slice().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");
    // storage_key_string.insert(0, '[');
    // storage_key_string.push(']');
    // let storage_key_literal = syn::parse_str::<ExprArray>(&storage_key_string).unwrap();

    static IOTA: AtomicI32 = AtomicI32::new(0);
    // temporary solution, we'll switch to 32 byte storage keys later
    let storage_key = IOTA.fetch_add(1, Relaxed);

    let expanded = quote! {
        // TODO: derive documents (#137)
        struct #variable_name;

        impl zink::Storage<#variable_type> for #variable_name {
            const STORAGE_KEY: i32 = #storage_key;

            fn get() -> #variable_type {
                unsafe {
                    zink::ffi::evm::sload(Self::STORAGE_KEY)
                }
            }

            fn set(value: #variable_type) {
                unsafe {
                    zink::ffi::evm::sstore(Self::STORAGE_KEY, value);
                }
            }
        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let expr: ItemType = syn::parse_str("pub type Counter = i32;").unwrap();
        assert_eq!(parse(expr).to_string().as_str(), "struct Counter ; impl zink :: Storage < i32 > for Counter { const STORAGE_KEY : i32 = 0i32 ; fn get () -> i32 { unsafe { zink :: ffi :: evm :: sload (Self :: STORAGE_KEY) } } fn set (value : i32) { unsafe { zink :: ffi :: evm :: sstore (Self :: STORAGE_KEY , value) ; } } }");
    }
}
