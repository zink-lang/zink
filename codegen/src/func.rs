//! Built-in functions for EVM
use opcodes::ShangHai as OpCode;

/// Function `select` from WASM which is not avaiable in EVM.
const SELECT: [OpCode; 6] = [
    OpCode::JUMPDEST,
    OpCode::POP,
    OpCode::PUSH1,
    OpCode::Data(0x06),
    OpCode::ADD,
    OpCode::JUMP,
];

/// Function `sload` from EVM which is not avaiable in WASM.
const SLOAD: [OpCode; 5] = [
    OpCode::JUMPDEST,
    OpCode::SLOAD,
    OpCode::Data(0x06),
    OpCode::ADD,
    OpCode::JUMP,
];

/// Function `sload` from EVM which is not avaiable in WASM.
const SSTORE: [OpCode; 5] = [
    OpCode::JUMPDEST,
    OpCode::SSTORE,
    OpCode::Data(0x06),
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
