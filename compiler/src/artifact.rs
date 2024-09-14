//! Zink compiler artifact

use crate::Config;
use anyhow::Result;
use zabi::Abi;
use zingen::{Buffer, Constructor};

/// Zink compiler artifact
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug)]
pub struct Artifact {
    /// Contract ABIs
    pub abi: Vec<Abi>,
    /// Compiler configuration.
    pub config: Config,
    /// Runtime bytecode of the contract.
    pub runtime_bytecode: Vec<u8>,
    /// Creation bytecode constructor
    #[cfg_attr(feature = "serde", serde(skip))]
    pub constructor: Constructor,
}

impl Artifact {
    /// Generate the creation bytecode just in time
    pub fn bytecode(&self) -> Result<Buffer> {
        let mut cst = self.constructor.clone();
        cst.finish(self.runtime_bytecode.clone().into())
            .map_err(Into::into)
    }
}
