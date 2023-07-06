//! EVM stack abstraction.

use std::collections::VecDeque;

/// The shadow stack used for compilation.
pub struct Stack {
    inner: VecDeque<[u8; 32]>,
}

impl Stack {
    /// Put a value on the top of the stack.
    pub fn push(&mut self, value: [u8; 32]) {
        self.inner.push_back(value);
    }

    /// Push values on the top of the stack.
    pub fn pushn<const S: usize>(&mut self, value: [[u8; 32]; S]) {
        self.inner.append(&mut value.into());
    }

    /// Pop a value from the top of the stack.
    pub fn pop(&mut self) -> Option<[u8; 32]> {
        self.inner.pop_back()
    }

    /// Pop values from the top of the stack.
    pub fn popn(&mut self, size: usize) -> Option<Vec<[u8; 32]>> {
        Some(self.inner.split_off(self.inner.len() - size).into())
    }
}
