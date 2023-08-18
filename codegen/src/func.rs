//! Built-in functions for EVM
use opcodes::ShangHai as OpCode;

/// Function `select`.
const SELECT: [OpCode; 6] = [
    OpCode::JUMPDEST,
    OpCode::POP,
    OpCode::PUSH1,
    OpCode::Data(0x06),
    OpCode::ADD,
    OpCode::JUMP,
];

/// Function selector.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Func {
    /// Run select.
    Select,
}

impl Func {
    /// Get the bytecode of the function.
    pub fn bytecode(&self) -> Vec<u8> {
        match self {
            Self::Select => SELECT,
        }
        .into_iter()
        .map(|op| op.into())
        .collect()
    }
}
