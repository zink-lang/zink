//! Local instructions

use crate::{CodeGen, Result};

impl CodeGen {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
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
    pub fn _local_set(&mut self, index: u32) -> Result<()> {
        let index = index as usize;
        let offset = self.locals.offset_of(index)?;

        self.masm.memory_write_at(&offset)?;
        Ok(())
    }

    /// This _local_tee is like _local_set, but it also returns the value.
    pub fn _local_tee(&mut self, _index: u32) -> Result<()> {
        todo!()
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
