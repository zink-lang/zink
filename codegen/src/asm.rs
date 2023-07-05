//! Low level assembler implementation for EVM.
//!
//! TODO: refactor this module with Result as outputs.

use crate::limits::BufferOffset;
use opcodes::{OpCode as _, ShangHai as OpCode};

/// Maximum size of a evm contract.
const BUFFER_LIMIT: usize = 0x6000;

/// Low level assembler implementation for EVM.
pub struct Assmbler {
    /// Buffer of the assembler.
    buffer: [u8; BUFFER_LIMIT],
    /// Offset of the buffer.
    offset: BufferOffset,
    /// Gas counter.
    ///
    /// This is used to calculate the gas cost of the generated code.
    ///
    /// TODO: use a more precise type, eq `u256`.
    gas: u128,
}

impl Default for Assmbler {
    fn default() -> Self {
        Self {
            buffer: [0; BUFFER_LIMIT],
            offset: 0.into(),
            gas: 0,
        }
    }
}

impl Assmbler {
    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// increment the gas counter.
    ///
    /// TODO: use number bigger than `u256` for throwing proper errors.
    pub fn increment_gas(&mut self, gas: u128) {
        self.gas += gas;
    }

    /// Emit a byte.
    pub fn emit(&mut self, byte: u8) {
        self.buffer[self.offset.0 as usize] = byte;
        self.offset += 1.into();
    }

    /// Emit a single opcodes.
    pub fn emit_op(&mut self, opcode: OpCode) {
        self.emit(opcode.into());
        self.increment_gas(opcode.gas().into());
    }

    /// Emit bytes.
    pub fn emits<const L: usize>(&mut self, bytes: [u8; L]) {
        let dst = self.offset.0 as usize + L;
        self.buffer[self.offset.0 as usize..dst].copy_from_slice(&bytes);

        // NOTE: This is safe because u16::MAX will reach the end of
        // the buffer definitely, zink compiler will panic on it.
        self.offset += (L as u16).into();
    }

    /// Emit opcodes.
    pub fn emit_opcodes(&mut self, opcodes: &[OpCode]) {
        for opcode in opcodes {
            self.emit_op(*opcode);
        }
    }

    /// Emit Add.
    pub fn add(&mut self) {
        self.emit_op(OpCode::ADD.into());
    }

    /// Place n bytes on stack.
    pub fn push<const S: u8>(&mut self) {
        match S {
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
            _ => unreachable!("Invalid push size"),
        }
    }
}
