//! Macro for the function selector.

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_quote, FnArg, ItemFn, Signature};
use zabi::Abi;

/// Mark the function as external.
///
/// TODO: Generate ABI for functions (#144)
pub fn external(mut item: ItemFn) -> TokenStream {
    item.sig.abi = Some(parse_quote! { extern "C" });
    item.attrs.push(parse_quote! { #[no_mangle] });
    item.attrs
        .push(parse_quote! { #[allow(improper_ctypes_definitions)] });

    let selector: ItemFn = {
        let func = item.sig.ident.clone().to_string();
        let ident = Ident::new(&(func.clone() + "_selector"), Span::call_site());
        let selector = parse_selector(&item.sig);
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

/// Hash function signature to EVM selector.
fn parse_selector(sig: &Signature) -> String {
    let args = sig.inputs.iter().map(|arg| match arg {
        FnArg::Typed(pat) => pat.ty.clone().into_token_stream().to_string(),
        _ => panic!(
            "Unsupported function argument: {:?}",
            arg.into_token_stream().to_string()
        ),
    });

    Abi {
        name: sig.ident.to_string(),
        inputs: args.collect(),
    }
    .to_hex()
    .expect("Failed to serialize ABI")
}
