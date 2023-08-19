//! Case handlers

use crate::{CodeGen, ControlStackFrame, ControlStackFrameType, Error, Result, ToLSBytes};

impl CodeGen {
    pub(crate) fn handle_empty_return(&mut self) -> Result<()> {
        self.masm._push0()?;
        self.masm._push0()?;
        self.masm.asm._return()?;

        Ok(())
    }

    /// Handle the end of the function.
    pub(crate) fn handle_return(&mut self) -> Result<()> {
        let results = self.env.results();
        tracing::debug!("handle return, results: {results:?}");

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

        let len = results.len() as u8;
        let sp = self.masm.sp();
        for i in 0..len {
            // TODO: arthmetic overflow.
            //
            // 2 is for PC & self.
            self.masm.swap(sp - i - 2)?;
        }

        tracing::debug!("cleaning frame stack, target: {}", len + 1);
        while self.masm.sp() > len + 1 {
            self.masm._drop()?;
        }

        // TODO: handle the length of results > u8::MAX.
        self.masm.shift_pc(len, false)?;
        self.masm.push(&[0x04])?;
        self.masm._add()?;
        self.masm._jump()?;

        Ok(())
    }

    /// Handle the popping of a frame.
    ///
    /// TODO: validate stack IO for all frames (#59)
    pub(crate) fn handle_frame_popping(&mut self, frame: ControlStackFrame) -> Result<()> {
        match frame.ty {
            ControlStackFrameType::If(true) => {
                // TODO: fix this for nested if-else.
                self.handle_return()
            }
            ControlStackFrameType::Block => self.masm._jumpdest(),
            ControlStackFrameType::Loop => Ok(()),
            _ => self.handle_jumpdest(frame.original_pc_offset),
        }
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
