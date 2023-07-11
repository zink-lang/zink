//! The shadow stack used for compilation.

use crate::{Error, Result};
use smallvec::{smallvec, SmallVec};
use std::fmt::{self, Debug, Formatter};

/// EVM stack limit in stack items.
const STACK_LIMIT: usize = 0xc;

/// The shadow stack used for compilation.
#[derive(Default)]
pub struct Stack {
    /// Inner stack.
    inner: SmallVec<[SmallVec<[u8; 32]>; STACK_LIMIT]>,
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
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Put a byte on the top of the stack.
    pub fn push(&mut self, byte: u8) -> Result<()> {
        self.inner.push(smallvec![byte]);
        if self.len() > STACK_LIMIT {
            return Err(Error::StackOverflow(self.len()));
        }

        Ok(())
    }

    /// Put n (n < 32) bytes on the top of the stack.
    pub fn pushn(&mut self, bytes: &[u8]) -> Result<()> {
        self.inner.push(bytes.into());
        if self.len() > STACK_LIMIT {
            return Err(Error::StackOverflow(self.len()));
        }

        Ok(())
    }

    /// Pop a value from the top of the stack.
    pub fn pop(&mut self) -> Result<SmallVec<[u8; 32]>> {
        let byte = self.inner.pop().ok_or(Error::StackUnderflow(self.len()))?;

        Ok(byte)
    }

    /// Pop values from the top of the stack.
    pub fn popn(&mut self, size: usize) -> Result<SmallVec<[SmallVec<[u8; 32]>; 32]>> {
        let len = self
            .len()
            .checked_sub(size)
            .ok_or(Error::StackUnderflow(self.len()))?;

        let popped = self.inner[len..].into();
        self.inner = self.inner[..len].into();
        Ok(popped)
    }
}
