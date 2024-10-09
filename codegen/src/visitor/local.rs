//! Local instructions

use crate::{wasm::ToLSBytes, Error, Function, Result};

impl Function {
    /// This instruction gets the value of a variable.
    pub fn _local_get(&mut self, local_index: u32) -> Result<()> {
        let local_index = local_index as usize;
        if (self.is_main && local_index < self.ty.params().len()) || self.abi.is_some() {
            // Parsing data from selector.
            self._local_get_calldata(local_index)
        } else {
            // Passing data between local functions.
            self._local_get_var(local_index)
        }
    }

    /// This instruction sets the value of a variable.
    pub fn _local_set(&mut self, local_index: u32) -> Result<()> {
        self.masm.push(&self.env.alloc(local_index))?;
        self.masm._mstore()?;

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
        let mut offset = self.locals.offset_of(local_index)?;
        if self.abi.is_some() {
            offset = (4 + local_index * 32).to_ls_bytes().to_vec().into();
        }

        self.masm.push(&offset)?;
        self.masm._calldataload()?;

        Ok(())
    }

    /// Local get for variables.
    fn _local_get_var(&mut self, local_index: usize) -> Result<()> {
        tracing::trace!("Local get variable: {local_index}");
        if local_index + 1 > self.locals.len() {
            // The local we want is not from function arguments
            return Err(Error::InvalidLocalIndex(local_index));
        }

        self.masm.push(&self.env.alloc(local_index as u32))?;
        self.masm._mload()?;
        Ok(())
    }
}
