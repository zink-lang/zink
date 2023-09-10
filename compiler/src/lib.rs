//! Zink compiler.
#![deny(missing_docs)]

pub use crate::{
    compiler::Compiler,
    result::{Error, Result},
};

mod compiler;
mod parser;
mod result;
