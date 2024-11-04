//! Revert macro

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::Ident;

/// Revert with message
pub fn parse(input: TokenStream) -> TokenStream {
    let message = input.to_string();
    let len = message.len() as i32;
    if len > 128 {
        panic!("Only support revert message less than 128 bytes atm.");
    }

    let lit = Literal::string(&message);
    let rev = Ident::new(
        &format!(
            "revert{}",
            match len {
                len if len > 96 => 4,
                len if len > 64 => 3,
                len if len > 32 => 2,
                len if len > 0 => 1,
                _ => panic!("Only support revert message less than 128 bytes atm."),
            },
        ),
        Span::call_site(),
    );

    quote! {
        zink::ffi::asm::#rev(#lit)
    }
    .into()
}
