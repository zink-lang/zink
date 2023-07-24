//! Program Relocations

use crate::{
    jump::{relocate, JumpTable},
    Buffer, Error, Result, ToLSBytes, BUFFER_LIMIT,
};
use opcodes::ShangHai as OpCode;
use std::collections::{BTreeMap, BTreeSet};

impl JumpTable {
    /// Relocate program counter to all registered labels.
    pub fn relocate(&mut self, buffer: &mut Buffer) -> Result<()> {
        let mut funcs = BTreeMap::default();
        while let Some((pc, jump)) = self.jump.pop_first() {
            let target = self.target(&jump)?;
            if jump.is_label() {
                let offset = relocate::pc(buffer, pc, target)?;

                if pc > target {
                    // TODO:
                    //
                    // 1. check this logic with more tests.
                    // 2. checked add offset.
                    //
                    // BUG: the target offset could be outdated since
                    // it will be modifed by the future relocations.
                    funcs.values_mut().for_each(|v| *v += offset as u16);
                } else {
                    self.shift_pc(pc, offset)?;
                }
            } else {
                funcs.insert(pc, target);
                let offset = relocate::offset(target)?;
                self.shift_label_pc(pc, offset)?;
            }
        }

        relocate::funcs(funcs, buffer)?;
        Ok(())
    }
}

/// Get the offset of the program counter for relocation.
fn offset(original_pc: u16) -> Result<u16> {
    let pc = original_pc;
    let mut offset = 0;

    // Update the target program counter
    {
        // The maximum size of the PC is 2 bytes, whatever PUSH1 or PUSH2
        // takes 1 more byte.
        offset += 1;

        // Update the program counter for the edge cases.
        //
        // Start from 0xff, the lowest significant bytes of the target
        // program counter will take 2 bytes instead of 1 byte.
        //
        // | PC   | PC BYTES | TARGET PC |
        // |------|----------|-----------|
        // | 0xfe | 1        |      0xff |
        // | 0xff | 2        |     0x101 |
        offset += if pc > 0xfe {
            // buffer.push(OpCode::PUSH2.into());
            2
        } else {
            // buffer.push(OpCode::PUSH1.into());
            1
        }
    }

    // Check PC range.
    if pc + offset > BUFFER_LIMIT as u16 {
        return Err(Error::InvalidPC((pc + offset) as usize));
    }

    Ok(offset)
}

/// Relocate program counter to buffer.
fn pc(buffer: &mut Buffer, original_pc: u16, target_pc: u16) -> Result<u16> {
    let original_pc = original_pc as usize;
    let mut pc = target_pc;
    let mut new_buffer: Buffer = buffer[..original_pc].into();
    let rest_buffer: Buffer = buffer[original_pc..].into();

    let offset = relocate::offset(original_pc as u16)?;
    if offset == 2 {
        new_buffer.push(OpCode::PUSH1.into());
    } else {
        new_buffer.push(OpCode::PUSH2.into());
    }

    pc += offset;
    tracing::debug!("run pc relocation: 0x{:x} -> 0x{:x}", original_pc, pc);
    let pc_offset = pc.to_ls_bytes();
    tracing::debug!("push bytes: {:x?} at {}", pc_offset, original_pc);
    new_buffer.extend_from_slice(&pc_offset);
    new_buffer.extend_from_slice(&rest_buffer);

    // Check buffer size.
    if new_buffer.len() > BUFFER_LIMIT {
        return Err(Error::BufferOverflow(new_buffer.len()));
    }

    *buffer = new_buffer;
    Ok(offset)
}

/// Relocate functions.
fn funcs(map: BTreeMap<u16, u16>, buffer: &mut Buffer) -> Result<()> {
    let values = map.values();
    let targets = values.clone().collect::<Vec<_>>();
    let mut targets_set = values.clone().collect::<BTreeSet<_>>();
    let mut final_targets = BTreeMap::<u16, u16>::new();
    while let Some(target) = targets_set.pop_first() {
        let count: u16 = targets
            .iter()
            .filter(|&&t| t == target)
            .count()
            .try_into()
            .map_err(|_| Error::InvalidPC(*target as usize))?;
        let target_usize = *target as usize;

        // dry run pc relocation.
        //
        // NOTE: skipping update the function PC for the first time bcz
        // it will be processed automatically in relocation.
        //
        // **DO NOT touch this again, it works.**
        let pc = relocate::offset(*target)?
            * (count.checked_sub(1).ok_or(Error::InvalidPC(target_usize))? as u16);

        // calculate the new target.
        final_targets.insert(
            *target,
            target
                .checked_add(pc)
                .ok_or(Error::InvalidPC(target_usize))?
                .try_into()
                .map_err(|_| Error::InvalidPC(target_usize + pc as usize))?,
        );
    }

    for (pc, target) in map.into_iter() {
        self::pc(
            buffer,
            pc,
            *final_targets
                .get(&target)
                .ok_or_else(|| Error::InvalidPC(target as usize))?,
        )?;
    }

    Ok(())
}
