//! System instructions

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
        // record the current program counter and
        // pass it to the callee function.
        self.masm._pc()?;

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
            .get(index as usize)
            .ok_or(Error::ImportedFuncNotFound(index))?;

        func.prelude(&mut self.masm)?;
        if func.is_embedded() {
            tracing::debug!("embed imported function {func:?} at index {index}");
            self.call_embedded(func)
        } else {
            tracing::debug!("call imported function {func:?} at index {index}");
            self.call_external(func)
        }
    }

    /// Call embedded functions
    fn call_embedded(&mut self, func: Func) -> Result<()> {
        match func {
            Func::Sstore => self.masm._sstore(),
            Func::Sload => self.masm._sload(),
            Func::Log0 => self.log0(),
            _ => Err(Error::UnsupportedHostFunc(func)),
        }
    }

    /// Call external functions
    pub fn call_external(&mut self, func: Func) -> Result<()> {
        self.table.ext(self.masm.pc_offset(), func);
        self.masm.decrement_sp(func.stack_in())?;
        self.masm._jump()?;
        self.masm._jumpdest()?;
        self.masm.increment_sp(func.stack_out())?;
        Ok(())
    }

    /// Logs a message with no topics.
    pub fn log0(&mut self) -> Result<()> {
        let buffer: Vec<u8> = self.masm.buffer().into();

        // Pop offset and size from the bytecode.
        let len = self.backtrace.popn(2);
        let data = &buffer[(buffer.len() - len - 1)..];
        tracing::debug!("log0 data: {:?}", data);
        *self.masm.buffer_mut() = buffer[..(buffer.len() - len)].into();

        // Parse offset.
        if !(0x5e..0x8f).contains(&data[0]) {
            todo!("handle invalid data offset in log");
        }
        let bytes_len = (data[0] - 0x5f) as usize;
        let offset = &data[1..(1 + bytes_len)];
        tracing::debug!("log0 offset: {:?}", offset);

        // Parse size.
        if !(0x5e..0x8f).contains(&data[bytes_len + 1]) {
            todo!("handle invalid data offset in log");
        }
        let size = &data[(bytes_len + 2)..];
        tracing::debug!("log0 size: {:?}", size);

        // Integrate with data section.

        Ok(())
    }
}
