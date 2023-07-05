//! EVM stack abstraction.
use std::ops::{Add, AddAssign};

/// EVM Stack limit.
const STACK_LIMIT: u16 = 1024;

/// Stack offset.
#[derive(Debug, Default, Clone, Copy)]
pub struct StackOffset(u16);

impl Add for StackOffset {
    type Output = StackOffset;

    fn add(self, other: StackOffset) -> StackOffset {
        // Check stack overflow here.
        let size = self.0 + other.0;
        if size > STACK_LIMIT {
            panic!("Stack overflow");
        }

        StackOffset(self.0 + other.0)
    }
}

impl AddAssign for StackOffset {
    fn add_assign(&mut self, other: StackOffset) {
        *self = *self + other;
    }
}

impl From<u16> for StackOffset {
    fn from(offset: u16) -> StackOffset {
        StackOffset(offset)
    }
}

impl Into<u16> for StackOffset {
    fn into(self) -> u16 {
        self.0
    }
}
