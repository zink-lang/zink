//! Zink compiler artifact

use crate::Config;
use zabi::Abi;

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
