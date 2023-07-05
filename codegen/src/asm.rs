//! Low level assembler implementation for EVM.

/// Low level assembler implementation for EVM.
#[derive(Default)]
pub struct Assmbler {
    /// Buffer of the assembler.
    buffer: Vec<u8>,
}

impl Assmbler {
    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Emit a single opcode.
    pub fn emit(&mut self, opcode: u8) {
        self.buffer.push(opcode);
    }
}
