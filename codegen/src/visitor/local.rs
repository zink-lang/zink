//! Local instructions

use crate::{CodeGen, Result};
use tracing::trace;

impl CodeGen {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        if !self.is_main {
            return Ok(());
        }

        trace!("local.get {}", local_index);
        if (local_index as usize) < self.env.params().len() {
            self.masm
                .push(&self.locals[local_index as usize].to_ls_bytes())?;
            self.masm._calldataload()?;
        } else {
            todo!("local.get {}", local_index);
        }

        Ok(())
    }

    /// This instruction sets the value of a variable.
    pub fn _local_set(&mut self, _index: u32) -> Result<()> {
        todo!()
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
