//! Jump table implementation.

use crate::{abi::ToLSBytes, Buffer, Error, Result, BUFFER_LIMIT};
use opcodes::ShangHai as OpCode;
use std::collections::{BTreeMap, BTreeSet};

/// Jump types
#[derive(Clone, Copy)]
pub enum Jump {
    /// Jump to the given label, the label here is the original
    /// program counter.
    Label(u16),
    /// Jump to function.
    Func(u32),
}

/// Jump table implementation.
#[derive(Default)]
pub struct JumpTable {
    /// Jump table.
    jump: BTreeMap<u16, Jump>,
    /// Function table.
    func: BTreeMap<u32, u16>,
}

impl JumpTable {
    /// Register a function.
    pub fn call(&mut self, pc: u16, func: u32) -> Result<()> {
        self.jump.insert(pc, Jump::Func(func));
        Ok(())
    }

    /// Register program counter to the function table.
    pub fn call_offset(&mut self, func: u32, offset: u16) -> Result<()> {
        if self.func.insert(func, offset).is_some() {
            return Err(Error::DuplicateFunc(func));
        }

        Ok(())
    }

    /// Register a label.
    pub fn label(&mut self, pc: u16, label: u16) {
        self.jump.insert(pc, Jump::Label(label));
    }

    /// Merge two jump tables.
    ///
    /// Merge other jump table into this one, update the program
    /// counter of the target jump table.
    pub fn merge(&mut self, mut table: Self, pc: u16) -> Result<()> {
        if pc != 0 {
            table.update_pc(pc as usize)?;
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

        Ok(())
    }

    /// Relocate program counter to all registered labels.
    pub fn relocate(&mut self, buffer: &mut Buffer) -> Result<()> {
        let mut funcs = BTreeMap::default();
        while let Some((pc, jump)) = self.jump.pop_first() {
            match jump {
                Jump::Label(label) => {
                    self.update_pc(Self::relocate_pc(buffer, pc as usize, label as usize)?)?;
                }
                Jump::Func(func) => {
                    let target = *self.func.get(&func).ok_or(Error::FuncNotFound(func))?;

                    funcs.insert(pc, target);

                    // dry run pc relocation here.
                    self.update_label_pc(Self::relocate_pc(
                        &mut buffer.clone(),
                        Default::default(),
                        target as usize,
                    )?)?;
                }
            }
        }

        // relocate functions.
        let values = funcs.values();
        let targets = values.clone().collect::<Vec<_>>();
        let mut targets_set = values.clone().collect::<BTreeSet<_>>();
        let mut final_targets = BTreeMap::<u16, u16>::new();
        while let Some(target) = targets_set.pop_first() {
            let count = targets.iter().filter(|&&t| t == target).count();
            let target_usize = *target as usize;

            // dry run pc relocation.
            //
            // NOTE: skipping update the function PC for the first time bcz
            // it will be processed automatically in relocation.
            let pc = Self::relocate_pc(&mut buffer.clone(), Default::default(), target_usize)?
                * count.checked_sub(1).ok_or(Error::InvalidPC(target_usize))?;

            // calculate the new target.
            final_targets.insert(
                *target,
                target_usize
                    .checked_add(pc)
                    .ok_or(Error::InvalidPC(target_usize))?
                    .try_into()
                    .map_err(|_| Error::InvalidPC(target_usize + pc))?,
            );
        }

        for (pc, target) in funcs.into_iter() {
            Self::relocate_pc(
                buffer,
                pc as usize,
                *final_targets
                    .get(&target)
                    .ok_or_else(|| Error::InvalidPC(target as usize))? as usize,
            )?;
        }

        Ok(())
    }

    /// Update program counter for labels.
    pub fn update_label_pc(&mut self, pc: usize) -> Result<()> {
        let pc: u16 = pc.try_into().map_err(|_| Error::InvalidPC(pc))?;
        self.jump = self
            .jump
            .iter()
            .map(|(k, v)| {
                let k = k + pc;
                if k > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(k as usize));
                }

                let v = match v {
                    Jump::Label(label) => Jump::Label(label + pc),
                    Jump::Func(func) => Jump::Func(*func),
                };

                Ok((k, v))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Update program counter for all items.
    pub fn update_pc(&mut self, pc: usize) -> Result<()> {
        self.update_label_pc(pc)?;

        let pc: u16 = pc.try_into().map_err(|_| Error::InvalidPC(pc))?;
        self.func = self
            .func
            .iter()
            .map(|(k, v)| {
                let v = v + pc;
                if v > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(v as usize));
                }
                Ok((*k, v))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Relocate program counter to buffer.
    pub fn relocate_pc(buffer: &mut Buffer, original_pc: usize, target_pc: usize) -> Result<usize> {
        let mut pc = target_pc;
        let mut new_buffer: Buffer = buffer[..original_pc].into();
        let rest_buffer: Buffer = buffer[original_pc..].into();

        // Update the target program counter
        {
            // The maximum size of the PC is 2 bytes, whatever PUSH1 or PUSH2
            // takes 1 more byte.
            pc += 1;

            // Update the program counter for the edge cases.
            //
            // Start from 0xff, the lowest significant bytes of the target
            // program counter will take 2 bytes instead of 1 byte.
            //
            // | PC   | PC BYTES | TARGET PC |
            // |------|----------|-----------|
            // | 0xfe | 1        |      0xff |
            // | 0xff | 2        |     0x101 |
            pc += if pc > 0xfe {
                new_buffer.push(OpCode::PUSH2.into());
                2
            } else {
                new_buffer.push(OpCode::PUSH1.into());
                1
            }
        }

        // Check PC range.
        if pc > BUFFER_LIMIT {
            return Err(Error::InvalidPC(pc));
        }

        new_buffer.extend_from_slice(&pc.to_ls_bytes());
        new_buffer.extend_from_slice(&rest_buffer);

        // Check buffer size.
        if new_buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(new_buffer.len()));
        }

        *buffer = new_buffer;
        Ok(pc - target_pc)
    }
}
