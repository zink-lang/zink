//! Revert macro

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream}, parse2, parse_macro_input, Data, DeriveInput, Expr, Fields, Ident, LitStr, Token, Type
};
use super::selector;

pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let error_variants = match input.data {
        Data::Enum(enum_data) => enum_data.variants,
        _ => panic!("Error derive macro only works on enums"),
    };

    let mut error_matches = vec![];
    let mut error_names = vec![];
    let mut error_signatures = vec![];

    for variant in error_variants {
        let variant_name = variant.ident;
        let signature = match variant.fields {
            Fields::Unnamed(fields) => {
                let types: Vec<Type> = fields.unnamed.iter()
                    .map(|f| f.ty.clone())
                    .collect();
                quote! { (#(#types),*) }
            },
            Fields::Unit => quote! {},
            Fields::Named(_) => panic!("Named fields are not supported in error variants"),
        };
        
        let selector = generate_error_selector(&variant_name.to_string());
        error_matches.push(quote! {
            #name::#variant_name #signature => #selector
        });
        error_names.push(variant_name.to_string());
        error_signatures.push(signature);
    }

    quote! {
        impl #name {
            pub fn selector(&self) -> [u8; 4] {
                match self {
                    #(#error_matches,)*
                }
            }

            pub fn get_abi() -> Vec<zink::abi::Error> {
                vec![
                    // #(zink::abi::Error {
                    //     name: #error_names.into(),
                    //     inputs: #error_signatures,
                    // },)*
                ]
            }
        }
    }.into()
}

fn generate_error_selector(name: &str) -> proc_macro2::TokenStream {
    quote! {
      
    }
}

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
