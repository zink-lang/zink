//! Code generation libary for the zink API

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod event;

/// Event logging interface
#[proc_macro_derive(Event)]
pub fn event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    event::parse(input)
}
