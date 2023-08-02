//! Local instructions

use crate::{CodeGen, Error, Result};
use smallvec::SmallVec;

impl CodeGen {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        if self.is_main {
            self._local_get_main(local_index)
        } else {
            self._local_get_callee(local_index)
        }
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

    /// Local get for main function
    fn _local_get_main(&mut self, local_index: u32) -> Result<()> {
        let local_index = local_index as usize;
        let offset = self.locals.offset_of(local_index)?;
        self.masm.push(&offset)?;

        if local_index < self.env.params().len() {
            // Get function parameters
            self.masm._calldataload()?;
        } else {
            // TODO: use stack instead of memory (#56)
            //
            // Get local variables
            self.masm._mload()?;
        }

        Ok(())
    }

    /// Local get for callee function
    fn _local_get_callee(&mut self, local_index: u32) -> Result<()> {
        let local_index = local_index as usize;
        if local_index + 1 > self.locals.len() {
            return Err(Error::InvalidLocalIndex(local_index));
        }

        let local = self.locals.get_mut(local_index)?;
        let expected_sp = {
            // preserved stack: [PC, self]
            local_index + 1
        };

        if local.sp != Some(expected_sp) {
            // TODO: SWAP locals
            unimplemented!("swap locals on stack")
        }

        Ok(())
    }
}
