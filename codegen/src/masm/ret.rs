//! Return handlers

use crate::{Error, MacroAssembler, Result, ToLSBytes};
use wasmparser::ValType;

impl MacroAssembler {
    /// Return with nothing.
    pub(crate) fn handle_empty_return(&mut self) -> Result<()> {
        self._push0()?;
        self._push0()?;
        self.asm._return()?;

        Ok(())
    }

    /// Handle the end of the main function.
    pub fn main_return(&mut self, results: &[ValType]) -> Result<()> {
        if results.is_empty() {
            return self.handle_empty_return();
        }

        let size = self.memory_write(results)?.size;
        let offset =
            self.mp_offset(|mp| mp.checked_sub(size).ok_or_else(|| Error::InvalidMP(0)))?;

        self.push(&size.to_ls_bytes())?;
        self.push(&offset)?;
        self.asm._return()?;
        Ok(())
    }

    /// Handle the return of a call.
    pub fn call_return(&mut self, results: &[ValType]) -> Result<()> {
        let len = results.len() as u8;
        let sp = self.sp();
        for i in 0..len {
            // TODO: arthmetic overflow.
            //
            // 2 is for PC & self.
            self.swap(sp - i - 2)?;
        }

        tracing::debug!("cleaning frame stack, target: {}", len + 1);
        while self.sp() > len + 1 {
            self._drop()?;
        }

        // TODO: handle the length of results > u8::MAX.
        self.shift_pc(len, false)?;

        // TODO: [PC, PUSHN, BYTES, JUMPDEST]
        self.push(&[0x04])?;
        self._add()?;
        self._jump()?;

        Ok(())
    }
}
