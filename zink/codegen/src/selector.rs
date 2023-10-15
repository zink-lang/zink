//! Macro for the function selector.

// use crate::utils::keccak;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_quote, FnArg, ItemFn, Signature};

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
        let doc = "EVM selector for the function `".to_string() + &func + "`";

        parse_quote! {
            #[no_mangle]
            #[doc(#doc)]
            pub extern "C" fn #ident() -> &'static str {
                #selector
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

    let mut input = sig.ident.to_string();
    input = input + "(" + &args.collect::<Vec<_>>().join(", ") + ")";
    input
}
