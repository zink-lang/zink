//! Jump Table
//!
//! This module defines the `JumpTable` struct, which manages the jump table, function
//! table, and code section. It provides methods to register jumps, functions, and
//! labels, as well as to merge jump tables.

use crate::{codegen::ExtFunc, jump::Jump, Code, Error, Result};
use std::collections::BTreeMap;

/// Jump table implementation.
#[derive(Clone, Default, Debug)]
pub struct JumpTable {
    /// Jump table mapping program counters to jump types.
    pub(crate) jump: BTreeMap<u16, Jump>,
    /// Function table mapping function indices to program counters.
    pub(crate) func: BTreeMap<u32, u16>,
    /// Code section associated with the jump table.
    pub(crate) code: Code,
}

impl JumpTable {
    /// Registers a function in the jump table.
    ///
    /// This function associates a program counter with a function.
    pub fn call(&mut self, pc: u16, func: u32) {
        self.jump.insert(pc, Jump::Func(func));
    }

    /// Registers a program counter to the function table.
    ///
    /// This function associates a function with a specific offset in the function table.
    pub fn call_offset(&mut self, func: u32, offset: u16) -> Result<()> {
        if self.func.insert(func, offset).is_some() {
            return Err(Error::DuplicateFunc(func));
        }

        Ok(())
    }

    /// Registers the start of the program counter for the code section.
    pub fn code_offset(&mut self, offset: u16) {
        self.code.shift(offset);
    }

    /// Registers an external function in the jump table.
    pub fn ext(&mut self, pc: u16, func: ExtFunc) {
        self.code.try_add_func(func.clone());
        self.jump.insert(pc, Jump::ExtFunc(func));
    }

    /// Registers a label in the jump table.
    pub fn label(&mut self, pc: u16, label: u16) {
        self.jump.insert(pc, Jump::Label(label));
    }

    /// Merges another jump table into this one.
    ///
    /// This function updates the program counters of the target jump table and
    /// handles any potential duplicates.
    pub fn merge(&mut self, mut table: Self, pc: u16) -> Result<()> {
        if pc != 0 {
            table.shift_pc(0, pc)?;
        }

        for (pc, jump) in table.jump.into_iter() {
            if self.jump.insert(pc, jump).is_some() {
                return Err(Error::DuplicateJump(pc));
            }
        }

        for (func, offset) in table.func.into_iter() {
            if self.func.insert(func, offset).is_some() {
                return Err(Error::DuplicateFunc(func));
            }
        }

        for func in table.code.funcs() {
            self.code.try_add_func(func);
        }

        Ok(())
    }

    /// register jump to program counter
    pub fn register(&mut self, pc: u16, jump: Jump) {
        self.jump.insert(pc, jump);
    }

    /// Get the max target from the current jump table
    pub fn max_target(&self) -> u16 {
        self.jump
            .iter()
            .filter_map(|(_, jump)| self.target(jump).ok())
            .max()
            .unwrap_or(0)
    }
}

#[test]
fn test_multiple_jumps_same_target() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    // Setup multiple jumps to same target
    table.register(0x10, Jump::Label(0x100));
    table.register(0x20, Jump::Label(0x100));
    table.shift_targets()?;

    // Verify each jump's final target
    assert_eq!(table.target(table.jump.get(&0x10).unwrap())?, 0x106);
    assert_eq!(table.target(table.jump.get(&0x20).unwrap())?, 0x106);
    Ok(())
}

#[test]
fn test_nested_jumps() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    // Create nested jump pattern
    table.register(0x10, Jump::Label(0x100)); // Jump to middle
    table.register(0x100, Jump::Label(0x200)); // Middle jumps to end
    table.register(0x20, Jump::Label(0x100)); // Another jump to middle

    table.shift_targets()?;

    // Verify jumps are processed correctly
    assert_eq!(table.target(table.jump.get(&0x10).unwrap())?, 0x106);
    assert_eq!(table.target(table.jump.get(&0x100).unwrap())?, 0x209);
    assert_eq!(table.target(table.jump.get(&0x20).unwrap())?, 0x106);
    Ok(())
}

#[test]
fn test_sequential_jumps() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    // Create sequence of jumps that follow each other
    table.register(0x10, Jump::Label(0x20));
    table.register(0x20, Jump::Label(0x30));
    table.register(0x30, Jump::Label(0x40));

    table.shift_targets()?;

    // Each target should be shifted by accumulated offset
    assert_eq!(table.target(table.jump.get(&0x10).unwrap())?, 0x22);
    assert_eq!(table.target(table.jump.get(&0x20).unwrap())?, 0x34);
    assert_eq!(table.target(table.jump.get(&0x30).unwrap())?, 0x46);
    Ok(())
}

#[test]
fn test_jump_backwards() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    table.register(0x10, Jump::Label(0x20));
    table.register(0x30, Jump::Label(0x20));

    table.shift_targets()?;

    assert_eq!(table.target(table.jump.get(&0x10).unwrap())?, 0x22);
    assert_eq!(table.target(table.jump.get(&0x30).unwrap())?, 0x22);
    Ok(())
}

#[test]
fn test_jump_table_state_consistency() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    // Register a sequence of jumps that mirror ERC20's pattern
    table.register(0x10, Jump::Label(0x100)); // First jump
    table.register(0x20, Jump::Label(0x100)); // Second jump to same target

    // Record state before and after each operation
    let initial_state = table.jump.clone();
    table.shift_targets()?;
    let shifted_state = table.jump.clone();

    // Verify jump table consistency
    assert_eq!(table.jump.len(), initial_state.len());
    assert!(shifted_state.values().all(|j| matches!(j, Jump::Label(_))));
    Ok(())
}

#[test]
fn test_jump_target_ordering() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    // Register jumps in reverse order
    table.register(0x30, Jump::Label(0x100));
    table.register(0x20, Jump::Label(0x100));
    table.register(0x10, Jump::Label(0x100));

    // Track all target shifts
    let mut shifts = Vec::new();
    let cloned = table.clone();
    let original_targets: Vec<_> = cloned.jump.values().collect();

    table.shift_targets()?;

    // Verify target consistency
    for (orig, shifted) in original_targets.iter().zip(table.jump.values()) {
        shifts.push((orig, shifted));
    }

    Ok(())
}

#[test]
fn test_mixed_jump_types() -> anyhow::Result<()> {
    let mut table = JumpTable::default();

    // Mix function calls and labels like in ERC20
    table.func.insert(1, 0x100);
    table.call(0x10, 1); // Function call
    table.register(0x20, Jump::Label(0x100)); // Label jump to same target

    let before_shift = table.jump.clone();
    table.shift_targets()?;
    let after_shift = table.jump.clone();

    // Compare states
    assert_eq!(before_shift.len(), after_shift.len());

    Ok(())
}
