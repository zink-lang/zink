//! Command `Build`.
use crate::utils::WasmBuilder;
use anyhow::{anyhow, Result};
use clap::Parser;
use etc::{Etc, FileSystem};
use std::{env, fs, path::PathBuf};
use zinkc::Compiler;

/// Build zink project to EVM bytecode.
#[derive(Debug, Parser)]
#[command(name = "build", version)]
pub struct Build {
    /// The path of the cargo project.
    pub input: Option<PathBuf>,
    /// Write output to \<filename\>
    #[clap(short, long, value_name = "filename")]
    pub output: Option<PathBuf>,
    /// Write output to compiler-chosen filename in \<dir\>
    #[clap(long, value_name = "dir")]
    pub out_dir: Option<PathBuf>,
    /// If enable dispatcher.
    #[clap(short, long)]
    pub dispatcher: bool,
}

impl Build {
    /// Run build
    pub fn run(&self) -> Result<()> {
        // Get and check the input.
        let input = if let Some(input) = self.input.as_ref() {
            input.clone()
        } else {
            env::current_dir()?
        };
        {
            if Etc::new(&input)?.find("Cargo.toml").is_err() {
                return Ok(());
            }

            if !input.is_dir() {
                return Err(anyhow!(
                    "Only support rust project directory as input for now"
                ));
            }
        }

        // Build the wasm.
        let mut builder = WasmBuilder::new(input)?;
        {
            if let Some(out_dir) = self.out_dir.as_ref() {
                builder.with_out_dir(out_dir);
            }

            if let Some(output) = self.output.as_ref() {
                builder.with_output(output);
            }

            builder.build()?;
        }

        // Compile the wasm to evm bytecode.
        let wasm = fs::read(builder.output()?)?;
        let bin = Compiler::default()
            .dispatcher(self.dispatcher)
            .compile(&wasm)?;
        let dst = builder.output()?.with_extension("bin");
        fs::write(dst, bin)?;

        Ok(())
    }
}
