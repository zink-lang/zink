//! call instructions

use crate::{CodeGen, Error, Func, Result};

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
    pub fn _call(&mut self, index: u32) -> Result<()> {
        // TODO: check the safety of the function index.
        let base = self.imports.len() as u32;

        if base > index {
            self.call_imported(index)
        } else {
            self.call_internal(base + index)
        }
    }

    /// Call internal functions
    fn call_internal(&mut self, index: u32) -> Result<()> {
        // record the current program counter and
        // pass it to the callee function.
        self.masm._pc()?;

        // Call an internal function.
        //
        // register the call index to the jump table.
        self.table.call(self.masm.pc_offset(), index);

        // jump to the callee function
        //
        // TODO: check the stack output.
        self.masm._jump()?;
        self.masm._jumpdest()?;

        Ok(())
    }

    /// Call imported functions
    fn call_imported(&mut self, index: u32) -> Result<()> {
        // call an imported function.
        //
        // register the imported function index to the jump table.
        let func = *self
            .imports
            .get(&index)
            .ok_or(Error::ImportedFuncNotFound(index))?;

        func.prelude(&mut self.masm)?;

        match func {
            Func::Sstore => self.masm._sstore(),
            Func::Sload => self.masm._sload(),
            Func::Log0 => self.log(0),
            Func::Log1 => self.log(1),
            Func::Log2 => self.log(2),
            Func::Log3 => self.log(3),
            Func::Log4 => self.log(4),
            _ => {
                tracing::error!("unsupported embedded function {func:?}");
                Err(Error::UnsupportedHostFunc(func))
            }
        }
    }
}
