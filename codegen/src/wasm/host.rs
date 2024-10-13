//! Host functions

use crate::{Error, Result};
use core::str::FromStr;
use opcodes::{OpCode as _, ShangHai as OpCode};

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
    /// check equal of two addresses
    AddressEq,
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
            ("asm", name) => {
                if name.starts_with("sload") {
                    Ok(Self::Evm(OpCode::SLOAD))
                } else {
                    Ok(Self::NoOp)
                }
            }
            ("evm", name) => Ok(Self::Evm(OpCode::from_str(name).map_err(|_| {
                tracing::error!("Failed to load host function: {:?}", import);
                Error::HostFuncNotFound(module.into(), name.into())
            })?)),
            ("zinkc", "emit_abi") => Ok(Self::EmitABI),
            ("zinkc", "address_eq") => Ok(Self::Evm(OpCode::EQ)),
            ("zinkc", "label_reserve_mem_32") => Ok(Self::Label(CompilerLabel::ReserveMemory32)),
            ("zinkc", "label_reserve_mem_64") => Ok(Self::Label(CompilerLabel::ReserveMemory64)),
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
