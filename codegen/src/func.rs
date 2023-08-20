//! Built-in functions for EVM
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

/// Function `sload` from EVM which is not available in WASM.
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
}

impl TryFrom<(&str, &str)> for Func {
    type Error = ();

    fn try_from(import: (&str, &str)) -> Result<Self, Self::Error> {
        match import {
            ("zink", "sload") => Ok(Self::Sload),
            ("zink", "sstore") => Ok(Self::Sstore),
            _ => Err(()),
        }
    }
}
