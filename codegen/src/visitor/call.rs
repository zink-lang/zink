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
        if self.imports.len() as u32 > index {
            self.call_imported(index)
        } else {
            self.call_internal(index)
        }
    }

    /// Call internal functions
    fn call_internal(&mut self, index: u32) -> Result<()> {
        tracing::debug!("call internal function: index={index}");
        // record the current program counter and
        // pass it to the callee function.
        self.table.offset(self.masm.pc_offset(), 6);
        self.masm.increment_sp(1)?;
        self.masm._jumpdest()?;

        // Call an internal function.
        //
        // register the call index to the jump table.
        //
        // TODO: support same pc different jumps. (#160)
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
        tracing::debug!("call imported function: index={index}");
        // call an imported function.
        //
        // register the imported function index to the jump table.
        let func = *self
            .imports
            .get(&index)
            .ok_or(Error::ImportedFuncNotFound(index))?;

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
