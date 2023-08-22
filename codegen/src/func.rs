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

/// Function selector.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Func {
    /// Run function select.
    Select,
    /// Run function sload.
    Sload,
    /// Run function sstore.
    Sstore,
}

impl Func {
    /// Stack input size.
    pub fn stack_in(&self) -> u8 {
        match self {
            Self::Select => 3,
            Self::Sload => 1,
            Self::Sstore => 2,
        }
    }

    /// Stack output size.
    pub fn stack_out(&self) -> u8 {
        match self {
            Self::Select => 1,
            Self::Sload => 1,
            Self::Sstore => 0,
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
            Self::Sload => masm._swap1(),
            Self::Sstore => masm._swap2(),
        }
    }

    /// Get the bytecode of the function.
    pub fn bytecode(&self) -> Vec<u8> {
        match self {
            Self::Select => SELECT.to_vec(),
            Self::Sload => SLOAD.to_vec(),
            Self::Sstore => SSTORE.to_vec(),
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
        match self {
            Self::Select => true,
            Self::Sload => true,
            Self::Sstore => true,
        }
    }
}

impl TryFrom<(&str, &str)> for Func {
    type Error = Error;

    fn try_from(import: (&str, &str)) -> Result<Self> {
        let (module, name) = import;
        match import {
            ("zink", "sload") => Ok(Self::Sload),
            ("zink", "sstore") => Ok(Self::Sstore),
            _ => Err(Error::HostFuncNotFound(module.into(), name.into())),
        }
    }
}
