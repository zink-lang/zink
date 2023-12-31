//! Code generation library for the zink API

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn, ItemType};

mod constructor;
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

/// Order-based storage macro.
/// Currently only i32 is supported
///
/// ```ignore
/// use zink::storage;
///
/// #[storage]
/// pub type Counter = i32;
/// ```
///
/// will generate:
///
/// ```ignore
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

/// Mark the function as constructor
#[proc_macro_attribute]
pub fn constructor(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    constructor::parse(input).into()
}
