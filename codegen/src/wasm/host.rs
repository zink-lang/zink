//! Host functions

use crate::{Error, Result};
use core::str::FromStr;
use opcodes::{OpCode as _, ShangHai as OpCode};

/// EVM built-in function.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HostFunc {
    /// EVM assemble operations.
    Evm(OpCode),
    // Zinkc helper functions
    //
    /// Emit ABI to the compiler.
    EmitABI,
}

impl HostFunc {
    /// Stack input size.
    pub fn stack_in(&self) -> u8 {
        match self {
            Self::Evm(op) => op.stack_in() as u8,
            _ => 0,
        }
    }

    /// Stack output size.
    pub fn stack_out(&self) -> u8 {
        match self {
            Self::Evm(op) => op.stack_out() as u8,
            _ => 0,
        }
    }
}

impl TryFrom<(&str, &str)> for HostFunc {
    type Error = Error;

    fn try_from(import: (&str, &str)) -> Result<Self> {
        let (module, name) = import;
        match import {
            ("evm", name) => {
                Ok(Self::Evm(OpCode::from_str(name).map_err(|_| {
                    Error::HostFuncNotFound(module.into(), name.into())
                })?))
            }
            ("zinkc", "emit_abi") => Ok(Self::EmitABI),
            _ => {
                tracing::warn!("Failed to load host function: {:?}", import);
                Err(Error::HostFuncNotFound(module.into(), name.into()))
            }
        }
    }
}
