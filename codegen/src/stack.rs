//! The shadow stack used for compilation.

use crate::{Error, Result};
use std::{
    collections::VecDeque,
    fmt::{self, Debug, Formatter},
};

/// EVM stack limit in bytes.
const STACK_LIMIT: u16 = 0x400;

/// The shadow stack used for compilation.
#[derive(Default)]
pub struct Stack {
    /// Inner stack.
    inner: VecDeque<u8>,
}

impl Debug for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:x?}", self.inner))
    }
}

impl Stack {
    /// If the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get the length of the stack.
    pub fn len(&self) -> u16 {
        self.inner.len() as u16
    }

    /// Put a byte on the top of the stack.
    pub fn push(&mut self, byte: u8) -> Result<()> {
        self.inner.push_back(byte);
        if self.len() > STACK_LIMIT {
            return Err(Error::StackOverflow(self.len()));
        }

        Ok(())
    }

    /// Put n (n < 32) bytes on the top of the stack.
    pub fn pushn(&mut self, bytes: &[u8]) -> Result<()> {
        self.inner.append(&mut bytes.to_vec().into());
        if self.len() > STACK_LIMIT {
            return Err(Error::StackOverflow(self.len()));
        }

        Ok(())
    }

    /// Pop a value from the top of the stack.
    pub fn pop(&mut self) -> Result<u8> {
        let byte = self
            .inner
            .pop_back()
            .ok_or(Error::StackUnderflow(self.len()))?;

        Ok(byte)
    }

    /// Pop values from the top of the stack.
    pub fn popn(&mut self, size: usize) -> Result<VecDeque<u8>> {
        Ok(self.inner.split_off(
            self.inner
                .len()
                .checked_sub(size)
                .ok_or(Error::StackUnderflow(self.len()))?,
        ))
    }
}
