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

    /// Checks if the target is a function call.
    pub fn is_call(&self) -> bool {
        !self.is_label()
    }
}

#[cfg(test)]
mod tests {
    use crate::jump::{Jump, JumpTable};
    use smallvec::smallvec;

    #[allow(unused)]
    fn init_tracing() {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .without_time()
            .compact()
            .try_init()
            .ok();
    }

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
        table.register(0x30, Jump::Label(0x40)); // Jump to label at 0x40

        assert_target_shift_vs_relocation(table)
    }

    #[test]
    fn test_multiple_internal_calls() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Simulate multiple functions calling _approve
        table.register(0x10, Jump::Label(0x100)); // approve() -> _approve
        table.register(0x20, Jump::Label(0x100)); // spend_allowance() -> _approve

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
    fn test_label_call_interaction() -> anyhow::Result<()> {
        init_tracing();
        let mut table = JumpTable::default();

        table.func.insert(1, 0x317);
        table.label(0x10, 0x12);
        table.call(0x11, 1);

        let mut buffer = smallvec![0; table.max_target() as usize];
        table.relocate(&mut buffer)?;

        assert_eq!(buffer[0x11], 0x17, "{buffer:?}");
        assert_eq!(buffer[0x14], 0x03, "{buffer:?}");
        assert_eq!(buffer[0x15], 0x1c, "{buffer:?}");
        Ok(())
    }

    #[test]
    fn test_large_target_offset_calculation() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Register a jump with target < 0xff
        table.register(0x10, Jump::Label(0x80));

        // Register a jump with target > 0xff
        table.register(0x20, Jump::Label(0x100));

        // Register a jump with target > 0xfff
        table.register(0x30, Jump::Label(0x1000));

        let mut buffer = smallvec![0; table.max_target() as usize];
        table.relocate(&mut buffer)?;

        // Check if offsets are correctly calculated
        // For target 0x80: PUSH1 (1 byte) + target (1 byte)
        // For target 0x100: PUSH2 (1 byte) + target (2 bytes)
        // For target 0x1000: PUSH2 (1 byte) + target (2 bytes)
        assert_eq!(buffer[0x11], 0x88); // Small target
        assert_eq!(buffer[0x23], 0x01); // First byte of large target
        assert_eq!(buffer[0x24], 0x08); // Second byte of large target
        assert_eq!(buffer[0x36], 0x10); // First byte of large target
        assert_eq!(buffer[0x37], 0x08); // Second byte of large target

        Ok(())
    }

    #[test]
    fn test_sequential_large_jumps() -> anyhow::Result<()> {
        let mut table = JumpTable::default();

        // Register multiple sequential jumps with increasing targets
        // This mirrors the ERC20 pattern where we have many functions
        for i in 0..20 {
            let target = 0x100 + (i * 0x20);
            table.register(0x10 + i, Jump::Label(target));
        }

        let mut buffer = smallvec![0; table.max_target() as usize];
        table.relocate(&mut buffer)?;

        // Check first jump (should use PUSH2)
        assert_eq!(buffer[0x10], 0x61); // PUSH2
        assert_eq!(buffer[0x11], 0x01); // First byte
        assert_eq!(buffer[0x12], 0x3c); // Second byte
        assert_eq!(0x013c, 0x100 + 20 * 3);

        // Check last jump (should still use PUSH2 but with adjusted offset)
        let last_idx = 0x10 + 19 + 19 * 3;
        assert_eq!(buffer[last_idx], 0x61); // PUSH2
        assert_eq!(buffer[last_idx + 1], 0x03); // First byte should be larger
        assert_eq!(buffer[last_idx + 2], 0x9c); // Second byte accounts for all previous jumps
        assert_eq!(0x039c, 0x100 + 0x20 * 19 + 20 * 3);

        Ok(())
    }
}
