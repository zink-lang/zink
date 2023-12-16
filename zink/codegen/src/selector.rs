//! Macro for the function selector.

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_quote, ItemFn};
use zabi::Abi;

/// Mark the function as external.
pub fn external(mut item: ItemFn) -> TokenStream {
    item.sig.abi = Some(parse_quote! { extern "C" });
    item.attrs.push(parse_quote! { #[no_mangle] });
    item.attrs
        .push(parse_quote! { #[allow(improper_ctypes_definitions)] });

    let selector: ItemFn = {
        let func = item.sig.ident.clone().to_string();
        let ident = Ident::new(&(func.clone() + "_selector"), Span::call_site());
        let selector = Abi::from(&item.sig).to_hex().expect("ABI is not supported");
        let selector_len = selector.len() as u32;
        let doc = " EVM selector for the function `".to_string() + &func + "`";

        parse_quote! {
            #[no_mangle]
            #[cfg(target_arch = "wasm32")]
            #[doc = #doc]
            pub extern "C" fn #ident() {
                unsafe {
                    zink::ffi::emit_abi(#selector.as_ptr() as u32, #selector_len);
                }
            }
        }
    };

    quote! {
        #item

        #selector
    }
    .into()
}
