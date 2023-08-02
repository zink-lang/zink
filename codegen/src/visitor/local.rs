//! Local instructions

use crate::{CodeGen, Error, Result};

impl CodeGen {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        let local_index = local_index as usize;
        if self.is_main && local_index < self.env.params().len() {
            self._local_get_calldata(local_index)
        } else {
            self._local_get_var(local_index)
        }
    }

    /// This instruction sets the value of a variable.
    pub fn _local_set(&mut self, local_index: u32) -> Result<()> {
        let index = local_index as usize;
        let sp = self.masm.sp();
        let local = self.locals.get_mut(index)?;
        let local_sp = if let Some(sp) = local.sp {
            sp as u8
        } else {
            local.sp = Some(sp as usize);
            return Ok(());
        };

        tracing::debug!("local_set: {} {} {}", index, sp, local_sp);
        if sp == local_sp {
            return Ok(());
        }

        self.masm.swap(sp - local_sp)?;
        self.masm._drop()?;

        Ok(())
    }

    /// This _local_tee is like _local_set, but it also returns the value
    /// on the stack, however, in our implementation, they are the same.
    pub fn _local_tee(&mut self, index: u32) -> Result<()> {
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

        let local = self.locals.get(local_index)?;
        let local_sp = local.sp.ok_or(Error::LocalNotOnStack(local_index))? as u8;
        let sp = self.masm.sp();

        if local_sp == sp || local_sp + 1 == sp {
            return Ok(());
        }

        tracing::trace!("local_get: {} {} {}", local_index, sp, local_sp);
        // TODO: Arthmetic checks
        self.masm.dup(sp - local_sp)?;
        Ok(())
    }
}
