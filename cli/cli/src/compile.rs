//! Command `Compile`.
#![cfg(feature = "zinkc")]

use anyhow::Result;
use clap::Parser;
use std::{env, fs, path::PathBuf};
use zinkc::Compiler;

/// Compile WASM to EVM bytecode.
#[derive(Debug, Parser)]
#[command(name = "build", version)]
pub struct Compile {
    /// The path of the wasm file.
    #[clap(value_name = "INPUT")]
    input: PathBuf,
    /// Write output to \<filename\>
    #[clap(short, long)]
    output: Option<PathBuf>,
    /// If enable dispatcher.
    #[clap(short, long)]
    dispatcher: bool,
}

impl Compile {
    /// Run compile.
    pub fn run(&self) -> Result<()> {
        let output = if let Some(output) = self.output.as_ref() {
            output.into()
        } else {
            env::current_dir()?.join(self.input.with_extension(""))
        };

        let bin = Compiler::default()
            .dispatcher(self.dispatcher)
            .compile(&fs::read(&self.input)?)?;

        fs::write(output, bin)?;
        Ok(())
    }
}
