//! Backtrace support for the code generation.

/// Backtrace implementation for the code generation.
///
/// TODO: full implementation #21
#[derive(Default)]
pub struct Backtrace {
    /// The length of each operand.
    len: Vec<usize>,
}

impl Backtrace {
    /// Pushes a new operand to the backtrace.
    pub fn push(&mut self, len: usize) {
        self.len.push(len);
    }

    /// Pops the last operand from the backtrace.
    pub fn pop(&mut self) -> usize {
        self.len.pop().unwrap_or_default()
    }

    pub fn popn(&mut self, n: usize) -> usize {
        let mut r: Vec<usize> = Default::default();

        while r.len() < n {
            r.push(self.pop())
        }

        r.into_iter().sum()
    }
}
