//! Local instructions

use crate::{CodeGen, Result, ToLSBytes};
// use tracing::trace;

impl CodeGen {
    /// Push a 32-bit integer value on the stack.
    pub fn _i32_const(&mut self, value: i32) -> Result<()> {
        // TODO:
        //
        // 1. push value to locals
        // 2. save value in memory.
        self.masm.push(value.to_ls_bytes().as_ref())?;
        Ok(())
    }

    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        if !self.is_main {
            return Ok(());
        }

        let local_index = local_index as usize;
        self.masm.push(&self.locals.offset_of(local_index))?;

        if local_index < self.env.params().len() {
            // Get function parameters
            self.masm._calldataload()?;
        } else {
            // Get local variables
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
