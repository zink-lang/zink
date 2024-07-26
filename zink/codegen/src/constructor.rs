//! Contract constructor.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemFn};

/// Parse function into contract constructor.
pub fn parse(input: ItemFn) -> TokenStream {
    let block = input.block;
    let inputs = input.sig.inputs;
    let constructor: ItemFn = parse_quote! {
        #[no_mangle]
        pub extern "C" fn constructor( #inputs ) {
            #block
        }
    };

    // constructor.attrs = input.attrs;
    constructor.into_token_stream()
}
