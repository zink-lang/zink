//! Assembly FFI.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::module_inception)]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
