//! Jump table implementation.

use crate::codegen::ExtFunc;
use core::fmt::Display;
pub use table::JumpTable;

mod pc;
mod relocate;
mod table;
mod target;

/// Jump types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Jump {
    /// offset to the program counter.
    Offset(u16),
    /// Jump to the given label, the label here
    /// is the original program counter.
    Label(u16),
    /// Jump to function.
    Func(u32),
    /// External function.
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
    /// If the target is a label.
    pub fn is_label(&self) -> bool {
        matches!(self, Jump::Label { .. })
    }

    /// If the target is fixed to offset of the program counter.
    pub fn is_offset(&self) -> bool {
        matches!(self, Jump::Offset(_))
    }

    /// If the target is a function call
    pub fn is_call(&self) -> bool {
        !self.is_label() && !self.is_offset()
    }
}
