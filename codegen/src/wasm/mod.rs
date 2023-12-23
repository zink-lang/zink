//! WASM related primitives.

mod host;
mod section;

pub use self::{
    host::HostFunc,
    section::{Exports, Imports},
};
