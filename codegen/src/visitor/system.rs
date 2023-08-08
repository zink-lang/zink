//! System instructions

use crate::{CodeGen, Result};

impl CodeGen {
    /// The call indirect instruction calls a function indirectly
    /// through an operand indexing into a table.
    pub fn _call_indirect(
        &mut self,
        _type_index: u32,
        _table_index: u32,
        _table_byte: u8,
    ) -> Result<()> {
        todo!()
    }

    /// The call instruction calls a function specified by its index.
    pub fn _call(&mut self, function_index: u32) -> Result<()> {
        // record the current program counter and
        // pass it to the callee function.
        self.masm._pc()?;

        // register the call index to the jump table.
        self.table.call(self.masm.pc_offset(), function_index);

        // jump to the callee function
        //
        // TODO: check the stack output.
        self.masm._jump()?;
        self.masm._jumpdest()?;
        Ok(())
    }
}
