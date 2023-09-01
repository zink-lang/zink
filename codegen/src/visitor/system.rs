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
        // record the current program counter and
        // pass it to the callee function.
        self.masm._pc()?;

        // register function to the jump table.
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
        let data_len = self.backtrace.popn(2);
        let data = &buffer[(buffer.len() - data_len)..];
        *self.masm.buffer_mut() = buffer[..(buffer.len() - data_len)].into();

        // Parse offset.
        //
        // PUSH0 0x5e
        // ..
        // PUSH32 0x8f
        if !(0x5e..0x8f).contains(&data[0]) {
            return Err(Error::InvalidDataOffset);
        }

        let offset_len = (data[0] - 0x5f) as usize;
        let offset = {
            let mut bytes = [0; 4];
            bytes[..offset_len].copy_from_slice(&data[1..(1 + offset_len)]);
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log0 offset: {:?}", offset);

        // Parse size.
        if !(0x5e..0x8f).contains(&data[offset_len + 1]) {
            return Err(Error::InvalidDataOffset);
        }
        let size = {
            let mut bytes = [0; 4];
            let size_bytes = &data[(offset_len + 2)..];
            bytes[..size_bytes.len()].copy_from_slice(&size_bytes);
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log0 size: {:?}", size);

        // Integrate with data section.

        Ok(())
    }
}
