//! Jump table implementation.
//!
//! This module defines the `Jump` enum and the `JumpTable` struct, which are used to manage
//! various types of jumps in the program, including offsets, labels, function calls, and
//! external functions.

use crate::codegen::ExtFunc;
use core::fmt::Display;
pub use table::JumpTable;

mod pc;
mod relocate;
mod table;
mod target;

/// Represents the different types of jumps in the program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Jump {
    /// Offset to the program counter.
    Offset(u16),
    /// Jump to a specific label, which corresponds to the original program counter.
    Label(u16),
    /// Jump to a function identified by its index.
    Func(u32),
    /// Jump to an external function.
    ExtFunc(ExtFunc),
}

impl Display for Jump {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Jump::Offset(offset) => write!(f, "Offset(0x{offset:x})"),
            Jump::Label(offset) => write!(f, "Label(0x{offset:x})"),
            Jump::Func(index) => write!(f, "Func({index})"),
            Jump::ExtFunc(_) => write!(f, "ExtFunc"),
        }
    }
}

impl Jump {
    /// Checks if the target is a label.
    pub fn is_label(&self) -> bool {
        matches!(self, Jump::Label { .. })
    }

    /// Checks if the target is a fixed offset of the program counter.
    pub fn is_offset(&self) -> bool {
        matches!(self, Jump::Offset(_))
    }

    /// Checks if the target is a function call.
    pub fn is_call(&self) -> bool {
        !self.is_label() && !self.is_offset()
    }
}
