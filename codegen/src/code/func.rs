//! External Function for the code section.
use opcodes::ShangHai as OpCode;

trait OpCodesToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

impl OpCodesToBytes for &[OpCode] {
    fn to_bytes(self) -> Vec<u8> {
        [&[OpCode::JUMPDEST], self]
            .concat()
            .iter()
            .map(|op| (*op).into())
            .collect()
    }
}

/// External function in code section.
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct ExtFunc {
    /// Stack input.
    pub stack_out: u8,
    /// Stack output.
    pub stack_in: u8,
    /// The bytecode of the external function.
    pub bytecode: Vec<u8>,
}

impl ExtFunc {
    /// Function select.
    pub fn select() -> Self {
        Self {
            stack_in: 2,
            stack_out: 1,
            bytecode: [
                OpCode::POP,
                OpCode::PUSH1,
                OpCode::Data(0x06),
                OpCode::ADD,
                OpCode::JUMP,
            ]
            .to_bytes(),
        }
    }
}
