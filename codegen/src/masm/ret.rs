//! Return handlers

use crate::{wasm::ToLSBytes, Error, MacroAssembler, Result};
use wasmparser::ValType;

impl MacroAssembler {
    /// Return with nothing.
    pub(crate) fn _handle_empty_return(&mut self) -> Result<()> {
        self._push0()?;
        self._push0()?;
        self.asm._return()?;

        Ok(())
    }

    /// Handle the end of the main function.
    pub fn main_return(&mut self, results: &[ValType]) -> Result<()> {
        if results.is_empty() {
            self._push0()?;
            self._push0()?;
            self.asm._return()?;
        } else {
            self._push0()?; // Offset 0
            self._mstore()?; // Store result
            self.push(&32u32.to_ls_bytes())?; // Size 32
            self._push0()?; // Offset 0
            self.asm._return()?;
        }

        Ok(())
    }

    /// Handle the return of a call.
    pub fn call_return(&mut self, results: &[ValType]) -> Result<()> {
        let expected_sp = results.len() as u16 + 1; // Results + return PC
        if self.sp() < expected_sp {
            return Err(Error::StackUnderflow {
                expected: expected_sp,
                found: self.sp(),
            });
        }

        while self.sp() > expected_sp {
            self._drop()?;
        }

        if !results.is_empty() {
            self._swap1()?; // Result over return PC, SP = expected_sp
        }
        self._jump()?; // Jump back to caller
        Ok(())
    }
}
