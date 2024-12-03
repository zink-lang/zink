//! Host functions

use crate::{Error, Result};
use anyhow::anyhow;
use core::str::FromStr;
use opcodes::{Cancun as OpCode, OpCode as _};

/// EVM built-in function.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HostFunc {
    /// EVM assemble operations.
    Evm(OpCode),
    /// No operations, this only covers `push_$ty` at the moment.
    NoOp,
    // Zinkc helper functions
    //
    /// Emit ABI to the compiler.
    EmitABI,
    /// Push u256 max to stack
    U256MAX,
    /// Revert messages with length of slots
    Revert(usize),
    /// Compiler labels
    Label(CompilerLabel),
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
            ("zinkc", name) => match name {
                "emit_abi" => Ok(Self::EmitABI),
                "u256_add" => Ok(Self::Evm(OpCode::ADD)),
                "u256_sub" => Ok(Self::Evm(OpCode::SUB)),
                "u256_lt" => Ok(Self::Evm(OpCode::LT)),
                "u256_max" => Ok(Self::U256MAX),
                "u256_addmod" => Ok(Self::Evm(OpCode::ADDMOD)),
                "u256_mulmod" => Ok(Self::Evm(OpCode::MULMOD)),
                "label_reserve_mem_32" => Ok(Self::Label(CompilerLabel::ReserveMemory32)),
                "label_reserve_mem_64" => Ok(Self::Label(CompilerLabel::ReserveMemory64)),
                _ => Err(Error::HostFuncNotFound(module.into(), name.into())),
            },
            ("evm", name) => Ok(Self::Evm(OpCode::from_str(name).map_err(|_| {
                tracing::error!("Failed to load host function: {:?}", import);
                Error::HostFuncNotFound(module.into(), name.into())
            })?)),
            ("asm", name) => match name {
                n if n.starts_with("sload") => Ok(Self::Evm(OpCode::SLOAD)),
                n if n.starts_with("tload") => Ok(Self::Evm(OpCode::TLOAD)),
                n if n.starts_with("revert") => {
                    let count = n.trim_start_matches("revert");
                    Ok(Self::Revert(count.parse().map_err(|e| anyhow!("{e}"))?))
                }
                n if n.starts_with("mulmod") => Ok(Self::Evm(OpCode::MULMOD)),
                n if n.starts_with("addmod") => Ok(Self::Evm(OpCode::ADDMOD)),
                _ => Ok(Self::NoOp),
            },
            ("bytes", instr) => match instr {
                push if push.starts_with("push_bytes") => Ok(Self::NoOp),
                sload if sload.starts_with("sload_bytes") => Ok(Self::Evm(OpCode::SLOAD)),
                eq if eq.ends_with("_eq") => Ok(Self::Evm(OpCode::EQ)),
                _ => {
                    tracing::warn!("Failed to load host function: {import:?} from module bytes");
                    Err(Error::HostFuncNotFound(module.into(), name.into()))
                }
            },
            _ => {
                tracing::warn!("Failed to load host function: {:?}", import);
                Err(Error::HostFuncNotFound(module.into(), name.into()))
            }
        }
    }
}

/// Labels in host functions
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum CompilerLabel {
    ReserveMemory32,
    ReserveMemory64,
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_addmod_mulmod_host_functions() -> anyhow::Result<()> {
        let addmod_func = HostFunc::try_from(("zinkc", "u256_addmod"))?;

        assert_eq!(addmod_func, HostFunc::Evm(OpCode::ADDMOD));

        // Test MULMOD host function conversion
        let mulmod_func = HostFunc::try_from(("zinkc", "u256_mulmod"));
        assert!(mulmod_func.is_ok());
        Ok(())
    }
}
