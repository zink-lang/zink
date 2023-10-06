//! Code generation library for the zink API

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemType};

mod event;
mod storage;

pub use storage::Storage;

/// Event logging interface
///
/// ```rust
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
/// ```rust
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

#[proc_macro_attribute]
pub fn storage(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemType);
    storage::parse(input).into()
}