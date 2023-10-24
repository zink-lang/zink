//! Backtrace support for the code generation.
use std::collections::BTreeMap;

/// Backtrace implementation for the code generation.
///
/// TODO: full implementation (#21)
#[derive(Debug, Default)]
pub struct Backtrace {
    /// Compiled instructions.
    ///
    /// TODO: Transform this into Opcodes. (#21)
    instrs: BTreeMap<usize, Vec<u8>>,
}

impl Backtrace {
    /// Pushes a new instruction set to the backtrace.
    pub fn push(&mut self, bytes: impl AsRef<[u8]>) {
        self.instrs.insert(self.instrs.len(), bytes.as_ref().into());
    }

    /// Pops the last instruction from the backtrace.
    pub fn pop(&mut self) -> Vec<u8> {
        self.instrs.pop_last().unwrap_or_default().1
    }

    /// Pop the last `n` operands from the backtrace.
    pub fn popn(&mut self, n: usize) -> Vec<Vec<u8>> {
        let mut r: Vec<Vec<u8>> = Default::default();

        while r.len() < n {
            r.push(self.pop())
        }

        r
    }
}
