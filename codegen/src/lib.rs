//! Code generation library for zink.

pub use crate::{
    isa::EvmIsa,
    result::{Error, Result},
};

mod isa;
mod result;
mod visitor;

/// The code generation abstraction.
pub struct CodeGen {}
