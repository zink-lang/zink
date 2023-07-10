//! The shadow stack used for compilation.

use smallvec::SmallVec;

use crate::{Error, Result};
use std::collections::VecDeque;

/// EVM stack limit in bytes.
const STACK_LIMIT: u16 = 0x400;

/// The shadow stack used for compilation.
#[derive(Default)]
pub struct Stack {
    /// Inner stack.
    inner: VecDeque<u8>,
    /// Stack pointer (0x00 ~ 0x400) for bytes.
    ///
    /// - the 32th byte represents [0x10]
    /// - the 256th byte represents [0xff]
    /// - the 257th byte represents [0xff, 0x01]
    /// - the 1024th byte represents [0xff; 1024]
    ///
    /// Save the stack pointer as cahce for the calculation
    /// of address and checks.
    pointer: u16,
}

impl Stack {
    /// Get the stack pointer.
    pub fn ptr(&self) -> u16 {
        self.pointer
    }

    /// Get the current stack address.
    pub fn address(&self) -> SmallVec<[u8; 32]> {
        let mut address = SmallVec::new();
        let mut reminder = self.pointer;
        while reminder > u8::MAX as u16 {
            address.push(u8::MAX);
            reminder -= u8::MAX as u16;
        }
        address.push(reminder as u8);
        address
    }

    /// Put a byte on the top of the stack.
    pub fn push(&mut self, byte: u8) -> Result<()> {
        self.inner.push_back(byte);
        self.increment_ptr(&[byte])?;
        Ok(())
    }

    /// Push bytes on the top of the stack.
    pub fn pushn<const S: usize>(&mut self, bytes: [u8; S]) -> Result<()> {
        self.inner.append(&mut bytes.into());
        self.increment_ptr(&bytes)?;
        Ok(())
    }

    /// Pop a value from the top of the stack.
    pub fn pop(&mut self) -> Result<u8> {
        let byte = self
            .inner
            .pop_back()
            .ok_or(Error::StackUnderflow(self.pointer))?;

        self.decrement_ptr(&[byte])?;
        Ok(byte)
    }

    /// Pop values from the top of the stack.
    pub fn popn(&mut self, size: usize) -> Result<Vec<u8>> {
        let bytes: Vec<u8> = self
            .inner
            .split_off(
                self.inner
                    .len()
                    .checked_sub(size)
                    .ok_or(Error::StackUnderflow(self.pointer))?,
            )
            .into();

        self.decrement_ptr(&bytes)?;
        Ok(bytes)
    }

    /// Increment the stack pointer.
    fn increment_ptr(&mut self, bytes: &[u8]) -> Result<()> {
        for byte in bytes {
            self.pointer += *byte as u16;
            if self.pointer > STACK_LIMIT {
                return Err(Error::StackOverflow(self.pointer));
            }
        }

        Ok(())
    }

    /// Decrement the stack pointer.
    fn decrement_ptr(&mut self, bytes: &[u8]) -> Result<()> {
        for byte in bytes {
            self.pointer
                .checked_sub(*byte as u16)
                .ok_or(Error::StackUnderflow(self.pointer))?;
        }

        Ok(())
    }
}
