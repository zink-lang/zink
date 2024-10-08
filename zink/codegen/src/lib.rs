//! Code generation library for the zink API

#![allow(unused)]
use proc_macro::TokenStream;
use syn::{parse_macro_input, Attribute, DeriveInput, ItemFn, ItemStruct};

mod event;
mod selector;
mod storage;

/// Event logging interface
///
/// ```ignore
/// use zink::Event;
///
/// /// A `Ping` event.
/// #[derive(Event)]
/// struct Ping;
///
/// #[no_mangle]
/// pub extern "C" fn log0() {
///     Ping.log0();
/// }
/// ```
///
/// will generate:
///
/// ```ignore
/// struct Ping;
///
/// impl zink::Event for Ping {
///     const NAME: &'static [u8] = b"Ping";
/// }
/// ```
#[proc_macro_derive(Event)]
pub fn event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    event::parse(input)
}

/// Declare on-chain storage
///
/// ```ignore
/// /// storage value
/// #[zink::storage(i32)]
/// pub struct Counter;
///
/// /// storage mapping
/// #[zink::storage(i32, i32)]
/// pub struct Mapping;
/// ```
#[proc_macro_attribute]
pub fn storage(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ty = storage::StorageType::from(attr);
    let input = parse_macro_input!(input as ItemStruct);
    storage::Storage::parse(ty, input)
}

/// Mark the function as an external entry point.
#[proc_macro_attribute]
pub fn external(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    selector::external(input)
}
