//! Local instructions

use crate::{CodeGen, Result};
use smallvec::SmallVec;

impl CodeGen {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        // let local_index = if self.is_main {
        //     local_index as usize
        // } else {
        //     (local_index as usize) + self.env.params().len() - 1
        // };
        //
        // let offset = self.locals.offset_of(local_index)?;
        // self.masm.push(&offset)?;
        //
        // if self.is_main && local_index < self.env.params().len() {
        //     self.masm._calldataload()?;
        // } else {
        //     self.masm._mload()?;
        // }
        //
        // Ok(())
        if !self.is_main {
            return Ok(());
        }

        let local_index = local_index as usize;
        let offset = self.locals.offset_of(local_index)?;
        self.masm.push(&offset)?;

        if local_index < self.env.params().len() {
            // Get function parameters
            self.masm._calldataload()?;
        } else {
            // Get local variables
            self.masm._mload()?;
        }

        Ok(())
    }

    /// This instruction sets the value of a variable.
    pub fn _local_set(&mut self, index: u32) -> Result<SmallVec<[u8; 32]>> {
        let index = index as usize;
        let offset = self.locals.offset_of(index)?;

        self.masm.memory_write_at(&offset)?;
        Ok(offset)
    }

    /// This _local_tee is like _local_set, but it also returns the value.
    pub fn _local_tee(&mut self, index: u32) -> Result<()> {
        let offset = self._local_set(index)?;

        self.masm.push(&offset)?;

        Ok(())
    }

    /// This instruction gets the value of a variable.
    pub fn _global_get(&mut self, _: u32) -> Result<()> {
        todo!()
    }

    /// This instruction sets the value of a variable.
    pub fn _global_set(&mut self, _: u32) -> Result<()> {
        todo!()
    }
}
