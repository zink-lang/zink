//! Low level assembler implementation for EVM.
//!
//! TODO: refactor this module with Result as outputs. (issue-21)

use crate::{Error, Result};
use opcodes::{OpCode as _, ShangHai as OpCode};
use smallvec::SmallVec;

/// Maximum size of a evm contract.
const BUFFER_LIMIT: usize = 0x6000;

/// Low level assembler implementation for EVM.
pub struct Assembler {
    /// Buffer of the assembler.
    buffer: SmallVec<[u8; BUFFER_LIMIT]>,
    /// Gas counter.
    ///
    /// This is used to calculate the gas cost of the generated code.
    ///
    /// TODO: use a more precise type, eq `u256`. (issue-20)
    gas: u128,
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            gas: 0,
        }
    }
}

impl Assembler {
    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// increment the gas counter.
    ///
    /// TODO: use number bigger than `u256` for throwing proper errors. (issue-21)
    pub fn increment_gas(&mut self, gas: u128) {
        self.gas += gas;
    }

    /// Emit a byte.
    pub fn emit(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    /// Emit bytes.
    pub fn emits(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    /// Emit a single opcodes.
    pub fn emit_op(&mut self, opcode: OpCode) {
        self.emit(opcode.into());
        self.increment_gas(opcode.gas().into());
    }

    /// Emit `ADD`
    pub fn add(&mut self) {
        self.emit_op(OpCode::ADD)
    }

    /// Emit `MSTORE`
    pub fn mstore(&mut self) {
        self.emit_op(OpCode::MSTORE)
    }

    /// Emit `MSTORE`
    pub fn ret(&mut self) {
        self.emit_op(OpCode::RETURN)
    }

    /// Emit `CALLDATALOAD`
    pub fn calldata_load(&mut self) {
        self.emit_op(OpCode::CALLDATALOAD)
    }

    /// Place n bytes on stack.
    pub fn push(&mut self, n: u8) -> Result<()> {
        match n {
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
            _ => return Err(Error::StackIndexOutOfRange(n)),
        }

        Ok(())
    }
}
