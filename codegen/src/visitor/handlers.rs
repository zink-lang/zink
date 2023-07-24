//! Case handlers

use crate::{CodeGen, Error, Result, ToLSBytes};

impl CodeGen {
    pub(crate) fn handle_empty_return(&mut self) -> Result<()> {
        self.masm._push0()?;
        self.masm._push0()?;
        self.masm.asm._return()?;

        Ok(())
    }

    /// Handle the end of the function.
    pub(crate) fn handle_return(&mut self) -> Result<()> {
        tracing::debug!("handle return");
        let results = self.env.results();
        if results.is_empty() {
            return self.handle_empty_return();
        }

        let size = self.masm.memory_write(results)?;
        let offset = self
            .masm
            .mp_offset(|mp| mp.checked_sub(size).ok_or_else(|| Error::InvalidMP(0)))?;

        self.masm.push(&size.to_ls_bytes())?;
        self.masm.push(&offset)?;
        self.masm.asm._return()?;
        Ok(())
    }

    /// Handle the return of a call.
    pub(crate) fn handle_call_return(&mut self) -> Result<()> {
        let results = self.env.results();
        tracing::debug!("handle call return: {:?}", results);

        // TODO: handle the length of results > u8::MAX.
        self.masm.shift_pc(results.len() as u8, false)?;
        self.masm.push(&[0x04])?;
        self.masm._add()?;
        self.masm._jump()?;

        Ok(())
    }

    /// Handle jumpdest.
    pub(crate) fn handle_jumpdest(&mut self, original_pc: u16) -> Result<()> {
        self.table.label(original_pc, self.masm.pc_offset());

        // TODO: Check the stack output and make decisions
        // how to handle the results.

        // Emit JUMPDEST after at the end of the control flow.
        self.masm._jumpdest()?;

        Ok(())
    }
}
