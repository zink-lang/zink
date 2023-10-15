//! Macro for the function selector.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemFn};

/// Mark the function as external.
pub fn external(mut item: ItemFn) -> TokenStream {
    item.sig.abi = Some(parse_quote! { extern "C" });
    item.attrs.push(parse_quote! { #[no_mangle] });

    quote! {
        #item
    }
    .into()
}

// /// Hash function signature to EVM selector.
// fn parse_selector(sig: &Signature) -> u32 {
//     let args = sig.inputs.iter().map(|arg| match arg {
//         FnArg::Typed(pat) => pat.ty.clone().into_token_stream().to_string(),
//         _ => panic!(
//             "Unsupported function argument: {:?}",
//             arg.into_token_stream().to_string()
//         ),
//     });
//
//     let mut input = sig.ident.to_string();
//     input = input + "(" + &args.collect::<Vec<_>>().join(", ") + ")";
//
//     let mut selector = [0u8; 4];
//     let hash = keccak(input.as_bytes());
//     selector.copy_from_slice(&hash[..4]);
//
//     u32::from_le_bytes(selector)
// }
