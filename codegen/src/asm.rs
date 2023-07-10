//! Low level assembler implementation for EVM.
//!
//! TODO: refactor this module with Result as outputs. (issue-21)

use crate::{abi::Offset, Error, Result, Stack};
use opcodes::{OpCode as _, ShangHai as OpCode};
use smallvec::SmallVec;

/// Maximum size of a evm bytecode in bytes.
pub const BUFFER_LIMIT: usize = 0x6000;

/// Maximum size of memory in evm in bytes.
pub const MEMORY_LIMIT: usize = 0x40;

/// Low level assembler implementation for EVM.
#[derive(Default)]
pub struct Assembler {
    /// Buffer of the assembler.
    buffer: SmallVec<[u8; BUFFER_LIMIT]>,
    /// Gas counter.
    ///
    /// This is used to calculate the gas cost of the generated code.
    ///
    /// TODO: use a more precise type, eq `u256`. (issue-20)
    gas: u128,
    /// Virtual memory for compilation.
    memory: SmallVec<[u8; MEMORY_LIMIT]>,
    /// Virtual stack for compilation.
    stack: Stack,
}

impl Assembler {
    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Mutable buffer of the assembler.
    pub fn buffer_mut(&mut self) -> &mut SmallVec<[u8; BUFFER_LIMIT]> {
        &mut self.buffer
    }

    /// Increment the gas counter.
    ///
    /// TODO: use number bigger than `u256` for throwing proper errors. (#21)
    pub fn increment_gas(&mut self, gas: u128) {
        self.gas += gas;
    }

    /// Emit a byte.
    pub fn emit(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    /// Emit n bytes.
    pub fn emitn(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    /// Emit a single opcodes.
    pub fn emit_op(&mut self, opcode: OpCode) {
        self.emit(opcode.into());
        self.increment_gas(opcode.gas().into());
    }

    /// Emit `ADD`
    pub fn add(&mut self) -> Result<()> {
        self.stack.pop()?;
        self.emit_op(OpCode::ADD);

        Ok(())
    }

    /// Emit `MSTORE`
    ///
    /// Use the current memory pointer as offset to store
    /// data in memory.
    pub fn mstore(&mut self) -> Result<u8> {
        let offset = self.memory.len();
        if offset > 32 {
            return Err(Error::MemoryOutOfBounds);
        }

        // push offset to stack.
        self.push(&offset.offset())?;

        // emit mstore.
        self.stack.popn(2)?;
        self.emit_op(OpCode::MSTORE);
        Ok(offset as u8)
    }

    /// Emit `JUMPI`
    pub fn jumpi(&mut self) {
        self.emit_op(OpCode::JUMP)
    }

    /// Emit `MSTORE`
    pub fn ret(&mut self) -> Result<()> {
        self.stack.popn(2)?;
        self.emit_op(OpCode::RETURN);
        Ok(())
    }

    /// Emit `CALLDATALOAD`
    pub fn calldata_load(&mut self) {
        self.emit_op(OpCode::CALLDATALOAD)
    }

    /// Place n bytes on stack.
    pub fn push(&mut self, bytes: &[u8]) -> Result<()> {
        let len = bytes.len();
        match len {
            0 => self.emit_op(OpCode::PUSH0),
            1 => self.emit_op(OpCode::PUSH1),
            2 => self.emit_op(OpCode::PUSH2),
            3 => self.emit_op(OpCode::PUSH3),
            4 => self.emit_op(OpCode::PUSH4),
            5 => self.emit_op(OpCode::PUSH5),
            6 => self.emit_op(OpCode::PUSH6),
            7 => self.emit_op(OpCode::PUSH7),
            8 => self.emit_op(OpCode::PUSH8),
            9 => self.emit_op(OpCode::PUSH9),
            10 => self.emit_op(OpCode::PUSH10),
            11 => self.emit_op(OpCode::PUSH11),
            12 => self.emit_op(OpCode::PUSH12),
            13 => self.emit_op(OpCode::PUSH13),
            14 => self.emit_op(OpCode::PUSH14),
            15 => self.emit_op(OpCode::PUSH15),
            16 => self.emit_op(OpCode::PUSH16),
            17 => self.emit_op(OpCode::PUSH17),
            18 => self.emit_op(OpCode::PUSH18),
            19 => self.emit_op(OpCode::PUSH19),
            20 => self.emit_op(OpCode::PUSH20),
            21 => self.emit_op(OpCode::PUSH21),
            22 => self.emit_op(OpCode::PUSH22),
            23 => self.emit_op(OpCode::PUSH23),
            24 => self.emit_op(OpCode::PUSH24),
            25 => self.emit_op(OpCode::PUSH25),
            26 => self.emit_op(OpCode::PUSH26),
            27 => self.emit_op(OpCode::PUSH27),
            28 => self.emit_op(OpCode::PUSH28),
            29 => self.emit_op(OpCode::PUSH29),
            30 => self.emit_op(OpCode::PUSH30),
            31 => self.emit_op(OpCode::PUSH31),
            32 => self.emit_op(OpCode::PUSH32),
            _ => return Err(Error::StackIndexOutOfRange(len as u8)),
        }

        // Place n bytes on stack.
        self.emitn(bytes);
        self.stack.pushn(bytes)?;

        Ok(())
    }
}
