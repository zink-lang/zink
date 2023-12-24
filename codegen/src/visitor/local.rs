//! Local instructions

use crate::{Error, Function, Result};

impl Function {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        let local_index = local_index as usize;
        if self.is_main && local_index < self.ty.params().len() {
            self._local_get_calldata(local_index)
        } else {
            self._local_get_var(local_index)
        }
    }

    /// This instruction sets the value of a variable.
    pub fn _local_set(&mut self, local_index: u32) -> Result<()> {
        let index = local_index as usize;
        let sp = self.masm.sp();
        let local = self.locals.get(index)?;
        let local_sp = local.sp as u8;

        tracing::trace!("local_set: {index} {local_sp} {sp}");
        self.masm.swap(sp - local_sp - 1)?;
        self.masm._drop()?;

        Ok(())
    }

    /// This _local_tee is like _local_set, but it also returns the value
    /// on the stack.
    pub fn _local_tee(&mut self, index: u32) -> Result<()> {
        self.masm._dup1()?;
        self._local_set(index)?;
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

    /// Local get from calldata.
    fn _local_get_calldata(&mut self, local_index: usize) -> Result<()> {
        let offset = self.locals.offset_of(local_index)?;
        self.masm.push(&offset)?;
        self.masm._calldataload()?;

        Ok(())
    }

    /// Local get for variables.
    fn _local_get_var(&mut self, local_index: usize) -> Result<()> {
        if local_index + 1 > self.locals.len() {
            return Err(Error::InvalidLocalIndex(local_index));
        }

        // If local is already on stack.
        if self.masm.buffer().len() == self.locals.len() + 1 {
            return Ok(());
        }

        tracing::debug!("buffer: {:?}", self.masm.buffer());

        let local = self.locals.get(local_index)?;
        let local_sp = local.sp as u8;
        let sp = self.masm.sp();

        tracing::trace!("local_get: {local_index} {local_sp} {sp}");

        // TODO: Arthmetic checks
        self.masm.dup(sp - local_sp)?;
        Ok(())
    }
}
