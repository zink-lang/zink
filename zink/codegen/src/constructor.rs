//! Contract constructor.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemFn};

/// Parse function into contract constructor.
pub fn parse(input: ItemFn) -> TokenStream {
    let block = input.block;
    let inputs = input.sig.inputs;
    let mut item: ItemFn = parse_quote! {
        #[no_mangle]
        pub extern "C" fn constructor( #inputs ) {
            #block
        }
    };

    item.attrs.push(parse_quote! { #[no_mangle] });
    item.attrs
        .push(parse_quote! { #[allow(improper_ctypes_definitions)] });
    item.into_token_stream()
}
