//! Zink compiler.
#![deny(missing_docs)]

pub use crate::{
    artifact::Artifact,
    compiler::Compiler,
    config::Config,
    result::{Error, Result},
};
pub use zingen::{Constructor, InitStorage};

mod artifact;
pub mod cli;
mod compiler;
mod config;
mod parser;
mod result;
pub mod utils;
