//! Zink compiler command line interface.
#![cfg(feature = "cli")]

use crate::{Compiler, Config};
use ccli::{clap, Parser};
use std::{env, fs, path::PathBuf};

/// Compile WASM to EVM bytecode.
#[derive(Debug, Parser)]
#[command(name = "zinkc")]
pub struct Compile {
    /// Write ABI to disk.
    #[clap(short, long)]
    abi: bool,
    /// The path of the wasm file.
    #[clap(value_name = "INPUT")]
    input: PathBuf,
    /// Write output to <filename>
    #[clap(short, long)]
    output: Option<PathBuf>,
    /// If enable dispatcher.
    #[clap(short, long)]
    dispatcher: bool,
}

impl Compile {
    /// Run compile.
    pub fn run(&self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output.as_ref() {
            output.into()
        } else {
            env::current_dir()?.join(self.input.with_extension(""))
        };

        let compiler = Compiler::new(Config::default().dispatcher(self.dispatcher));
        let artifact = compiler.compile(&fs::read(&self.input)?)?;

        output.parent().map(fs::create_dir_all);
        fs::write(&output, artifact.bytecode)?;

        if !self.abi {
            return Ok(());
        }

        let abi = output
            .parent()
            .ok_or_else(|| anyhow::anyhow!("invalid output path: {output:?}"))?
            .join(
                output
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("invalid file name: {output:?}"))?,
            )
            .with_extension("abi.json");

        fs::write(abi, serde_json::to_string_pretty(&artifact.abi)?)?;
        Ok(())
    }
}
