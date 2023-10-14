//! Code generation library for the zink API

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn, ItemType};

mod event;
mod selector;
mod storage;
mod utils;

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

/// Order-based storage macro.
/// Currently only i32 is supported
///
/// ```rust
/// use zink::storage;
///
/// #[storage]
/// pub type Counter = i32;
/// ```
///
/// will generate:
///
/// ```rust
/// struct Counter;
///
/// impl zink::Storage<i32> for Counter {
///     // if this macro were the second one in the project, this key would be 1i32
///     const STORAGE_KEY: i32 = 0i32;
///
///     fn get() -> i32 {
///         zink::ffi::evm::sload(Self::STORAGE_KEY)
///     }
///
///     fn set(value: i32) {
///         zink::ffi::evm::sstore(Self::STORAGE_KEY, value);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn storage(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemType);
    storage::parse(input).into()
}

/// Mark the function as an external entry point.
#[proc_macro_attribute]
pub fn external(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    selector::external(input)
}
