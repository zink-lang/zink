//! Revert macro

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse2, Expr, Ident, LitStr, Token,
};

/// Revert with message
pub fn parse(input: LitStr) -> TokenStream {
    let message = input.value();
    let len = message.len() as i32;
    if len > 128 {
        panic!("Only support revert message less than 128 bytes atm.");
    }

    // TODO: handle the string correctly
    let lit = Literal::string(&message.replace("\"", ""));
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
        unsafe { zink::ffi::asm::#rev(#lit) }
    }
    .into()
}

/// Parse assert macro
pub fn parse_assert(input: AssertInput) -> TokenStream {
    let cond = input.cond;
    let revert: Expr = syn::parse2(
        parse(
            input
                .message
                .unwrap_or(LitStr::new("unknown error", Span::call_site())),
        )
        .into(),
    )
    .expect("Invalid revert message");

    quote! {
        if !#cond {
            #revert
        }
    }
    .into()
}

/// Assert input
pub struct AssertInput {
    pub cond: Expr,
    pub comma: Token![,],
    pub message: Option<LitStr>,
}

impl Parse for AssertInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AssertInput {
            cond: input.parse()?,
            comma: input.parse()?,
            message: input.parse()?,
        })
    }
}
