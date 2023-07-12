//! Low level assembler implementation for EVM.
//!
//! TODO: refactor this module with Result as outputs. (issue-21)

use crate::{Error, Result};
use opcodes::{for_each_shanghai_operator, OpCode as _, ShangHai as OpCode};
use smallvec::SmallVec;

/// Maximum size of a evm bytecode in bytes.
pub const BUFFER_LIMIT: usize = 0x6000;

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
    /// Memory pointer, maximum 32, 64-bit words.
    pub mp: u8,
    /// Stack pointer, maximum 12 items.
    sp: u8,
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

    /// Increment stack pointer
    pub fn increment_sp(&mut self, items: u8) -> Result<()> {
        self.sp += items;
        if self.sp > 12 {
            return Err(Error::StackOverflow(self.sp));
        }

        Ok(())
    }

    /// Decrement stack pointer
    pub fn decrement_sp(&mut self, items: u8) -> Result<()> {
        self.sp = self
            .sp
            .checked_sub(items)
            .ok_or(Error::StackUnderflow(self.sp, items))?;
        Ok(())
    }

    /// Increment memory pointer
    pub fn increment_mp(&mut self, offset: u8) -> Result<()> {
        self.mp += offset;
        if self.mp > 32 {
            return Err(Error::MemoryOutOfBounds);
        }

        Ok(())
    }

    /// Decrement memory pointer
    pub fn decrement_mp(&mut self, offset: u8) -> Result<()> {
        self.mp = self
            .mp
            .checked_sub(offset)
            .ok_or(Error::MemoryOutOfBounds)?;
        Ok(())
    }

    /// Emit a byte.
    pub fn emit(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    /// Emit n bytes.
    pub fn emitn(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    /// Emit a single opcode.
    ///
    /// Mock the stack input and ouput for checking
    /// the stack usages.
    pub fn emit_op(&mut self, opcode: OpCode) -> Result<()> {
        self.decrement_sp(opcode.stack_in() as u8)?;
        self.emit(opcode.into());
        self.increment_gas(opcode.gas().into());
        self.increment_sp(opcode.stack_out() as u8)?;

        Ok(())
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
        }?;

        self.emitn(bytes);
        Ok(())
    }
}

macro_rules! impl_opcodes {
    ($($name:ident => $opcode:ident),+) => {
        $(
            #[doc = concat!(" Emit", stringify!($opcode))]
            pub fn $name(&mut self) -> Result<()> {
                self.emit_op(OpCode::$opcode)?;
                Ok(())
            }
        )*
    };
}

impl Assembler {
    for_each_shanghai_operator!(impl_opcodes);
}
