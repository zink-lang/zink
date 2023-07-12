//! The shadow stack used for compilation.

use crate::{Error, Result};
use smallvec::SmallVec;
use std::fmt::{self, Debug, Formatter};

/// EVM stack limit in stack items.
const STACK_LIMIT: usize = 0xc;

/// Stack item.
pub type StackItem = SmallVec<[u8; 32]>;

/// The shadow stack used for compilation.
#[derive(Default)]
pub struct Stack {
    /// Inner stack.
    inner: SmallVec<[StackItem; STACK_LIMIT]>,
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

    /// Push an item on the top of the stack.
    pub fn push(&mut self, item: StackItem) -> Result<()> {
        self.inner.push(item);
        if self.len() > STACK_LIMIT {
            return Err(Error::StackOverflow(self.len()));
        }

        Ok(())
    }

    /// Put n items on the top of the stack.
    pub fn pushn(&mut self, mut items: SmallVec<[StackItem; STACK_LIMIT]>) -> Result<()> {
        self.inner.append(&mut items);
        if self.len() > STACK_LIMIT {
            return Err(Error::StackOverflow(self.len()));
        }

        Ok(())
    }

    /// Pop an item from the top of the stack.
    pub fn pop(&mut self) -> Result<StackItem> {
        let byte = self
            .inner
            .pop()
            .ok_or(Error::StackUnderflow(self.len(), 1))?;

        Ok(byte)
    }

    /// Pop items from the top of the stack.
    pub fn popn(&mut self, size: usize) -> Result<SmallVec<[StackItem; STACK_LIMIT]>> {
        let len = self
            .len()
            .checked_sub(size)
            .ok_or(Error::StackUnderflow(self.len(), size))?;

        let popped = self.inner[len..].into();
        self.inner = self.inner[..len].into();
        Ok(popped)
    }
}
