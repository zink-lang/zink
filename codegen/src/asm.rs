//! Low level assembler implementation for EVM.
//!
//! TODO: refactor this module with Result as outputs. (issue-21)

use crate::{Buffer, Error, Result};
use opcodes::{for_each_shanghai_operator, OpCode as _, ShangHai as OpCode};

/// Low level assembler implementation for EVM.
pub struct Assembler {
    /// Buffer of the assembler.
    buffer: Buffer,
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
    /// New assembler
    pub fn new(sp: u8) -> Self {
        Self {
            buffer: Buffer::new(),
            gas: 0,
            mp: 0,
            sp,
        }
    }

    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Mutable buffer of the assembler.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
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
