//! Zink compiler.

pub use crate::{
    buffer::Buffer,
    compiler::Compiler,
    result::{Error, Result},
};

mod buffer;
mod compiler;
mod result;
