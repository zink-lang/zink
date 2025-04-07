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
use wasmparser::ValType;

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

        // Validate caller has enough stack items
        if self.masm.sp() < *params as u16 {
            return Err(Error::StackUnderflow {
                expected: *params as u16,
                found: self.masm.sp(),
            });
        }

        // Store parameters in memory and register the call index in the jump table.
        for i in (0..*params).rev() {
            tracing::trace!("Storing local at {} for function {index}", i + reserved);
            self.masm.push(&((i + reserved) * 0x20).to_ls_bytes())?;
            self.masm._mstore()?;
        }

        // Push return address
        let return_pc = self.masm.pc() + 3; // After JUMPDEST and JUMP
        self.masm.push(&return_pc.to_ls_bytes())?;
        self.table.label(self.masm.pc(), return_pc);
        self.masm._jumpdest()?;
        self.table.call(self.masm.pc(), index);
        self.masm._jump()?;

        // Post-call landing point
        self.masm._jumpdest()?;
        let _ = self.masm.set_sp(*results as u16 + 1); // Results + return PC
        Ok(())
    }

    ///
    pub fn call_return(&mut self, results: &[ValType]) -> Result<()> {
        tracing::trace!("call_return: results={:?}, sp={}", results, self.masm.sp());
        let expected_sp = results.len() as u16 + 1; // Results + return PC
        if self.masm.sp() < expected_sp {
            return Err(Error::StackUnderflow {
                expected: expected_sp,
                found: self.masm.sp(),
            });
        }

        if !results.is_empty() {
            self.masm._push0()?;
            self.masm._mstore()?;
            self.masm._swap1()?;
        }
        self.masm._jump()?;
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
