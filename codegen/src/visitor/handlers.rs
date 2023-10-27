//! Case handlers

use crate::{CodeGen, ControlStackFrame, ControlStackFrameType, Result};

impl CodeGen {
    /// Handle the end of the function.
    pub(crate) fn handle_return(&mut self) -> Result<()> {
        let results = self.env.results();
        tracing::trace!("handle return, results: {results:?}");

        self.masm.main_return(results)
    }

    /// Handle the return of a call.
    pub(crate) fn handle_call_return(&mut self) -> Result<()> {
        let results = self.env.results();
        tracing::trace!("handle call return: {:?}", results);

        self.masm.call_return(results)
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
