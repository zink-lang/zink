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

#[cfg(test)]
mod tests {
    use crate::jump::{Jump, JumpTable};
    use smallvec::smallvec;

    fn assert_target_shift_vs_relocation(mut table: JumpTable) -> anyhow::Result<()> {
        // Calculate expected buffer size based on the maximum target
        let mut buffer = smallvec![0; table.max_target() as usize];

        // Perform target shifts
        table.shift_targets()?;

        // Find the maximum target after shifts
        let max_target = table.max_target();

        // Perform relocation
        table.relocate(&mut buffer)?;
        assert_eq!(buffer.len(), max_target as usize);
        Ok(())
    }

    #[test]
    fn test_target_shift_vs_relocation() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Register jumps with known offsets and labels
        table.register(0x10, Jump::Label(0x20)); // Jump to label at 0x20
        table.register(0x20, Jump::Offset(0x10)); // Offset jump forward by 0x10
        table.register(0x30, Jump::Label(0x40)); // Jump to label at 0x40

        assert_target_shift_vs_relocation(table)
    }

    #[test]
    fn test_multiple_internal_calls() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Simulate multiple functions calling _approve
        table.register(0x10, Jump::Label(0x100)); // approve() -> _approve
        table.register(0x20, Jump::Label(0x100)); // spend_allowance() -> _approve
        table.register(0x100, Jump::Offset(0x30)); // _approve implementation

        assert_target_shift_vs_relocation(table)
    }

    #[test]
    fn test_nested_internal_calls() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Simulate transfer_from calling both _spend_allowance and _transfer
        table.register(0x10, Jump::Label(0x100)); // transfer_from -> _spend_allowance
        table.register(0x100, Jump::Label(0x200)); // _spend_allowance -> _approve
        table.register(0x20, Jump::Label(0x300)); // transfer_from -> _transfer
        table.register(0x300, Jump::Label(0x400)); // _transfer -> _update

        assert_target_shift_vs_relocation(table)
    }

    #[test]
    fn test_conditional_jumps() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Simulate the conditional logic in _spend_allowance
        table.register(0x10, Jump::Label(0x100)); // Entry point
        table.register(0x20, Jump::Label(0x200)); // If branch
        table.register(0x30, Jump::Label(0x300)); // Else branch
        table.register(0x100, Jump::Offset(0x50)); // Condition check
        table.register(0x200, Jump::Label(0x400)); // Call to _approve

        assert_target_shift_vs_relocation(table)
    }
}
