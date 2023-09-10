//! Jump table implementation.

pub use self::{code::Code, table::JumpTable};
use crate::Func;

mod code;
mod pc;
mod relocate;
mod table;

/// Jump types
#[derive(Clone, Copy, Debug)]
pub enum Jump {
    /// Jump to the given label, the label here is the original
    /// program counter.
    Label(u16),
    /// Jump to function.
    Func(u32),
    /// External function.
    ExtFunc(Func),
}

impl Jump {
    /// If the target is a label.
    pub fn is_label(&self) -> bool {
        matches!(self, Jump::Label(_))
    }
}
