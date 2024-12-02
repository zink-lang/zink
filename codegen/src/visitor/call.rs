//! Call Instructions
//!
//! This module provides functionality for calling functions, both internal and imported,
//! within the execution environment. It handles the setup of the call stack, manages
//! parameters, and ensures that the program counter is correctly adjusted for function
//! calls.

use crate::{
    wasm::{HostFunc, ToLSBytes},
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
        let (params, results) = self.env.funcs.get(&index).unwrap_or(&(0, 0));

        // Prepare the stack structure for the function call.
        // The stack will be structured as follows:
        // [ ..,
        //   <PC>,                         // The current program counter
        //   params[SWAP],                 // Swap the parameters for the call
        //   params[PUSH, SLOT, MSTORE],   // Push parameters to the stack
        //   {(PUSH, PC), JUMP, JUMPDEST}   // Prepare for the jump to the callee
        // ]
        let base_offset = 5 + ((params + reserved) * 0x20).saturating_sub(0xff) / 0x20;

        // Move the PC before the parameters in the stack.
        self.table.offset(
            self.masm.pc_offset(),
            base_offset as u16 + 4 * (*params as u16),
        );
        self.masm.increment_sp(1)?;

        // Adjust the stack to place the PC before the parameters.
        self.masm.shift_stack(*params as u8, true)?;

        // Store parameters in memory and register the call index in the jump table.
        for i in (0..*params).rev() {
            tracing::trace!("Storing local at {} for function {index}", i + reserved);
            self.masm.push(&((i + reserved) * 0x20).to_ls_bytes())?;
            self.masm._mstore()?;
        }

        // Register the call index in the jump table.
        self.table.call(self.masm.pc_offset(), index);

        // Jump to the callee function.
        self.masm._jump()?;
        self.masm._jumpdest()?;

        // Adjust the stack pointer for the results.
        self.masm.increment_sp(*results as u8)?;
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
