//! Revert macro

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;

/// Revert with message
pub fn parse(input: TokenStream) -> TokenStream {
    let message = input.to_string();
    let len = message.len() as i32;
    let lit = Literal::string(&message);

    quote! {
        zink::ffi::asm::revert(len, #lit)
    }
    .into()
}
