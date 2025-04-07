//! Low level assembler implementation for EVM.
//!
//! TODO: refactor this module with Result as outputs. (issue-21)

use crate::{Buffer, Error, Result};
use opcodes::{for_each_cancun_operator, Cancun as OpCode, OpCode as _};

const MAX_STACK_SIZE: u16 = 1024;

/// Low level assembler implementation for EVM.
#[derive(Default, Clone, Debug)]
pub struct Assembler {
    /// Buffer of the assembler.
    buffer: Buffer,
    /// Gas counter.
    ///
    /// This is used to calculate the gas cost of the generated code.
    ///
    /// TODO: use a more precise type, eq `u256`. (issue-20)
    gas: u128,
    /// Memory pointer for byte offset.
    pub mp: usize,
    /// Stack pointer, maximum `MAX_STACK_SIZE` items.
    pub sp: u16,
}

impl Assembler {
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
    pub fn increment_sp(&mut self, items: u16) -> Result<()> {
        if items == 0 {
            return Ok(());
        }

        tracing::trace!(
            "increment stack pointer {}.add({items}) -> {}",
            self.sp,
            self.sp + items
        );
        self.sp = self.sp.checked_add(items).ok_or(Error::StackOverflow {
            expected: self.sp,      // Current SP before overflow
            found: self.sp + items, // Attempted SP after increment
        })?;

        if self.sp > MAX_STACK_SIZE {
            return Err(Error::StackOverflow {
                expected: MAX_STACK_SIZE,
                found: self.sp,
            });
        }

        Ok(())
    }

    /// Decrement stack pointer
    pub fn decrement_sp(&mut self, items: u16) -> Result<()> {
        if items == 0 {
            return Ok(());
        }

        tracing::trace!(
            "decrement stack pointer {}.sub({items}) -> {}",
            self.sp,
            self.sp - items
        );
        self.sp = if self.sp == items {
            0
        } else {
            self.sp.checked_sub(items).ok_or(Error::StackUnderflow {
                expected: items, // Minimum required items
                found: self.sp,  // Current items available
            })?
        };

        Ok(())
    }

    /// Increment memory pointer
    pub fn increment_mp(&mut self, offset: usize) -> Result<()> {
        self.mp = self
            .mp
            .checked_add(offset)
            .ok_or(Error::MemoryOutOfBounds)?;

        Ok(())
    }

    /// Decrement memory pointer
    pub fn decrement_mp(&mut self, offset: usize) -> Result<()> {
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
    /// Mock the stack input and output for checking
    /// the stack usages.
    pub fn emit_op(&mut self, opcode: OpCode) -> Result<()> {
        tracing::trace!("emit opcode: {:?}", opcode);
        self.decrement_sp(opcode.stack_in())?;
        self.emit(opcode.into());
        self.increment_gas(opcode.gas().into());
        self.increment_sp(opcode.stack_out())?;

        Ok(())
    }
}

macro_rules! impl_opcodes {
    ($($name:ident => $opcode:ident),+) => {
        $(
            #[doc = concat!(" Emit ", stringify!($opcode))]
            pub fn $name(&mut self) -> Result<()> {
                self.emit_op(OpCode::$opcode)?;
                Ok(())
            }
        )*
    };
}

/// Basic instruction implementations
impl Assembler {
    for_each_cancun_operator!(impl_opcodes);
}
