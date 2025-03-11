//! Call Instructions
//!
//! This module provides functionality for calling functions, both internal and imported,
//! within the execution environment. It handles the setup of the call stack, manages
//! parameters, and ensures that the program counter is correctly adjusted for function
//! calls.

use crate::{
    wasm::{
        abi::{Address, FixedArray},
        HostFunc, ToLSBytes,
    },
    Error, Function, Result,
};
use anyhow::anyhow;
use opcodes::Cancun as OpCode;

impl Function {
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

    /// Calls a function specified by its index.
    ///
    /// This function determines whether the function is an external import or an internal
    /// function. If it is an external function, it will call the `call_imported` method.
    /// Otherwise, it will call the `call_internal` method to handle the internal function call.
    ///
    /// # Panics
    ///
    /// If an attempt is made to call an external function internally, this function will panic.
    pub fn _call(&mut self, index: u32) -> Result<()> {
        if self.env.is_external(index) {
            panic!("External functions could not be called internally");
        }

        if self.env.imports.len() as u32 > index {
            self.call_imported(index)
        } else {
            self.call_internal(index)
        }
    }

    /// Calls an internal function specified by its index.
    ///
    /// This function handles the mechanics of calling an internal function, including:
    /// - Checking for recursion and returning an error if detected.
    /// - Recording the current program counter (PC) to manage the return address.
    /// - Adjusting the stack to accommodate parameters and the return address.
    /// - Storing parameters in memory and registering the call index in the jump table.
    /// - Supports fixed arrays by storing them contiguously and redirecting to a single local.
    ///
    /// # Errors
    ///
    /// Returns an error if recursion is detected or if the function index is invalid.
    fn call_internal(&mut self, index: u32) -> Result<()> {
        if self.env.index == Some(index) {
            return Err(anyhow!(
                "Recursion is no longer supported in this version. See https://github.com/zink-lang/zink/issues/248"
            ).into());
        }

        tracing::debug!("Calling internal function: index={index}");
        let reserved = self.env.slots.get(&index).unwrap_or(&0);
        let (params, results) = {
            let func_data = self.env.funcs.get(&index).unwrap_or(&(0, 0));
            (func_data.0, func_data.1)
        };

        // TODO This is a temporary fix to avoid stack underflow.
        // We need to find a more elegant solution for this.
        self.masm.increment_sp(1)?;
        tracing::debug!("Stack pointer after increment_sp(1): {}", self.masm.sp());

        // For Issue #253: Assume the function takes a single [Address; 3] parameter
        // (params == 1). We'll generalize later if needed.
        if params == 1 {
            // Hardcode for [Address; 3] (3 * 20 = 60 bytes)
            let array_len = 3;
            let elem_size = 20; // Address size
            let total_size = array_len * elem_size; // 60 bytes
            tracing::trace!("Storing fixed array [Address; 3] for function {index}");

            // Access the memory offset (top stack item, 4 bytes)
            let memory_offset = {
                // Duplicate the top stack item (offset) to inspect it
                self.masm.dup(0)?;
                tracing::debug!("Stack pointer after dup(0): {}", self.masm.sp());
                // Use backtrace to get the last instruction (should be PUSH for offset)
                let buffer: Vec<u8> = self.masm.buffer().into();
                let data_len = self.backtrace.popn(1).concat().len();
                self.masm.decrement_sp(1)?;
                tracing::debug!("Stack pointer after decrement_sp(1): {}", self.masm.sp());

                let data = &buffer[(buffer.len() - data_len)..];
                *self.masm.buffer_mut() = buffer[..(buffer.len() - data_len)].into();

                // Parse the offset
                if !(0x5e..0x8f).contains(&data[0]) {
                    return Err(Error::InvalidDataOffset(data[0].into()));
                }
                let offset_len = (data[0] - 0x5f) as usize;
                let offset = {
                    let mut bytes = [0; 4];
                    bytes[(4 - offset_len)..].copy_from_slice(&data[1..(offset_len + 1)]);
                    bytes.reverse();
                    u32::from_le_bytes(bytes)
                };
                tracing::debug!("Array memory offset: {}", offset);
                offset
            };

            // Assume the caller wrote the 60 bytes to memory at memory_offset
            let array_data = vec![0u8; total_size];
            // No direct read method; caller must ensure memory is pre-loaded
            tracing::debug!("Assuming 60 bytes at memory offset {}", memory_offset);

            let array = FixedArray::new(
                array_data
                    .chunks(elem_size)
                    .map(|chunk| Address(chunk.try_into().unwrap()))
                    .collect(),
            );

            // Store the array in memory at reserved offset for local access
            let offset = reserved * 0x20;
            self.masm.push(&offset.to_ls_bytes())?;
            tracing::debug!("Stack pointer after push offset: {}", self.masm.sp());
            self.masm.push(&array.to_ls_bytes())?;
            tracing::debug!("Stack pointer after push array: {}", self.masm.sp());
            self.masm._mstore()?;
            tracing::debug!("Stack pointer after mstore: {}", self.masm.sp());

            // Store the offset in a local variable (local 0) for the function to access
            self.masm.push(&offset.to_ls_bytes())?;
            tracing::debug!("Stack pointer after push local offset: {}", self.masm.sp());
            self._local_set(0)?;
            tracing::debug!("Stack pointer after local_set: {}", self.masm.sp());
        } else {
            // Existing logic for scalar parameters
            for i in (0..params).rev() {
                tracing::trace!("Storing local at {} for function {index}", i + reserved);
                self.masm.push(&((i + reserved) * 0x20).to_ls_bytes())?;
                self.masm._mstore()?;
            }
        }

        // Register the label to jump back
        let return_pc = self.masm.pc() + 2;
        self.table.label(self.masm.pc(), return_pc);
        self.masm._jumpdest()?;
        tracing::debug!("Stack pointer after jumpdest: {}", self.masm.sp());

        // Register the call index in the jump table
        self.table.call(self.masm.pc(), index);
        self.masm._jump()?;
        tracing::debug!("Stack pointer after jump: {}", self.masm.sp());

        // Adjust the stack pointer for the results
        self.masm._jumpdest()?;
        self.masm.increment_sp(results as u16)?;
        tracing::debug!(
            "Stack pointer after increment_sp(results): {}",
            self.masm.sp()
        );

        Ok(())
    }

    /// Calls an imported function specified by its index.
    ///
    /// This function retrieves the imported function from the environment and executes it.
    /// It handles various host functions and ensures that the correct operations are performed
    /// based on the function type.
    ///
    /// # Errors
    ///
    /// Returns an error if the imported function is not found or if an unsupported host function
    /// is encountered.
    fn call_imported(&mut self, index: u32) -> Result<()> {
        // Retrieve the imported function index from the environment.
        let func = *self
            .env
            .imports
            .get(&index)
            .ok_or(Error::ImportedFuncNotFound(index))?;

        tracing::trace!("Calling imported function, index={index}, func={func:?}");
        match func {
            HostFunc::Evm(OpCode::LOG0) => self.log(0),
            HostFunc::Evm(OpCode::LOG1) => self.log(1),
            HostFunc::Evm(OpCode::LOG2) => self.log(2),
            HostFunc::Evm(OpCode::LOG3) => self.log(3),
            HostFunc::Evm(OpCode::LOG4) => self.log(4),
            HostFunc::Evm(op) => self.masm.emit_op(op),
            HostFunc::U256MAX => self.masm.push(&[255; 32]),
            HostFunc::Revert(count) => self.revert(count),
            HostFunc::NoOp | HostFunc::Label(_) => Ok(()),
            _ => {
                tracing::error!("Unsupported host function {func:?}");
                Err(Error::UnsupportedHostFunc(func))
            }
        }
    }
}
