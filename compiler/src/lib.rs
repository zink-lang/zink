//! Zink compiler.
#![deny(missing_docs)]

pub use crate::{
    compiler::Compiler,
    config::Config,
    result::{Error, Result},
};

pub mod cli;
mod compiler;
mod config;
mod parser;
mod result;
