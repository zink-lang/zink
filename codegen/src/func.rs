//! Built-in functions for EVM
use crate::{Error, MacroAssembler, Result};
use opcodes::ShangHai as OpCode;

/// Selects one of its first two operands based on whether
/// its third operand is zero or not.
const SELECT: [OpCode; 6] = [
    OpCode::JUMPDEST,
    OpCode::POP,
    OpCode::PUSH1,
    OpCode::Data(0x06),
    OpCode::ADD,
    OpCode::JUMP,
];

/// Function `sload` from EVM which is not available in WASM.
const SLOAD: [OpCode; 7] = [
    OpCode::JUMPDEST,
    OpCode::SLOAD,
    OpCode::SWAP1,
    OpCode::PUSH1,
    OpCode::Data(0x05),
    OpCode::ADD,
    OpCode::JUMP,
];

/// Function `sstore` from EVM which is not available in WASM.
const SSTORE: [OpCode; 6] = [
    OpCode::JUMPDEST,
    OpCode::SSTORE,
    OpCode::PUSH1,
    OpCode::Data(0x05),
    OpCode::ADD,
    OpCode::JUMP,
];

/// EVM built-in function.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Func {
    /// Run function select.
    Select,
    /// Run function sload.
    Sload,
    /// Run function sstore.
    Sstore,
    /// Run function log0.
    Log0,
    /// Run function log1.
    Log1,
    /// Run function log2.
    Log2,
    /// Run function log3.
    Log3,
    /// Run function log4.
    Log4,
}

impl Func {
    /// Stack input size.
    pub fn stack_in(&self) -> u8 {
        match self {
            Self::Select => 3,
            Self::Sload => 1,
            Self::Sstore => 2,
            Self::Log0 => 2,
            Self::Log1 => 4,
            Self::Log2 => 6,
            Self::Log3 => 8,
            Self::Log4 => 10,
        }
    }

    /// Stack output size.
    pub fn stack_out(&self) -> u8 {
        match self {
            Self::Select => 1,
            Self::Sload => 1,
            Self::Sstore => 0,
            Self::Log0 => 0,
            Self::Log1 => 0,
            Self::Log2 => 0,
            Self::Log3 => 0,
            Self::Log4 => 0,
        }
    }

    /// Pre-processing for the function.
    pub fn prelude(&self, masm: &mut MacroAssembler) -> Result<()> {
        match self {
            Self::Select => {
                masm._pc()?;
                masm._swap2()?;
                masm._swap1()
            }
            Self::Sstore => masm._swap1(),
            _ => Ok(()),
        }
    }

    /// Get the bytecode of the function.
    pub fn bytecode(&self) -> Vec<u8> {
        match self {
            Self::Select => SELECT.to_vec(),
            Self::Sload => SLOAD.to_vec(),
            Self::Sstore => SSTORE.to_vec(),
            _ => unimplemented!("not implemented for {:?}", self),
        }
        .into_iter()
        .map(|op| op.into())
        .collect()
    }

    /// If this function should be embedded in
    /// the main code.
    ///
    /// TODO: return `false` for functions that
    /// are necessary to just stay in the code
    /// section #109
    pub fn is_embedded(&self) -> bool {
        true
    }
}

impl TryFrom<(&str, &str)> for Func {
    type Error = Error;

    fn try_from(import: (&str, &str)) -> Result<Self> {
        let (module, name) = import;
        // NOTE: `select` is not external call
        // so we don't need to check process it
        // here
        match import {
            ("evm", "sload") => Ok(Self::Sload),
            ("evm", "sstore") => Ok(Self::Sstore),
            ("evm", "log0") => Ok(Self::Log0),
            ("evm", "log1") => Ok(Self::Log1),
            ("evm", "log2") => Ok(Self::Log2),
            ("evm", "log3") => Ok(Self::Log3),
            ("evm", "log4") => Ok(Self::Log4),
            _ => {
                tracing::error!("Failed to load host function: {:?}", import);
                Err(Error::HostFuncNotFound(module.into(), name.into()))
            }
        }
    }
}
