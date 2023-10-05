//! Event interface generation

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitByteStr};

/// Expand the event interface
pub fn parse(item: DeriveInput) -> TokenStream {
    let name = LitByteStr::new(item.ident.to_string().as_bytes(), Span::call_site().into());
    let ident = item.ident;

    let expanded = quote! {
        impl zink::Event for #ident {
            const NAME: &'static [u8] = #name;
        }
    };

    expanded.into()
}
