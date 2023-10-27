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
