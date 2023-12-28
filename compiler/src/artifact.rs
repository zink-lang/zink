//! Zink compiler artifact

use crate::{Compiler, Config};
use wasmparser::FuncType;
use zabi::Abi;
use zingen::Constructor;

/// Zink compiler artifact
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug)]
pub struct Artifact {
    /// Contract ABIs
    pub abi: Vec<Abi>,
    /// Bytecode of the contract.
    pub bytecode: Vec<u8>,
    /// Compiler configuration.
    pub config: Config,
    /// Runtime bytecode of the contract.
    pub runtime_bytecode: Vec<u8>,
}

impl TryFrom<(Compiler, Option<FuncType>)> for Artifact {
    type Error = anyhow::Error;

    fn try_from(
        (compiler, constructor): (Compiler, Option<FuncType>),
    ) -> Result<Self, Self::Error> {
        let Compiler {
            abi,
            buffer,
            config,
            ..
        } = compiler;

        let bytecode = Constructor::new(constructor, buffer.clone())?
            .finish()?
            .to_vec();

        Ok(Self {
            abi,
            bytecode,
            config,
            runtime_bytecode: buffer.to_vec(),
        })
    }
}
