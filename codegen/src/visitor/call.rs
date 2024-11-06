//! call instructions

use crate::{
    wasm::{HostFunc, ToLSBytes},
    Error, Function, Result,
};
use anyhow::anyhow;
use opcodes::ShangHai as OpCode;

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

    /// The call instruction calls a function specified by its index.
    pub fn _call(&mut self, index: u32) -> Result<()> {
        if self.env.is_external(index) {
            // TODO: throw with error
            panic!("External functions could not be called internally");
        }

        if self.env.imports.len() as u32 > index {
            self.call_imported(index)
        } else {
            self.call_internal(index)
        }
    }

    /// Call internal functions
    fn call_internal(&mut self, index: u32) -> Result<()> {
        if self.env.index == Some(index) {
            return Err(anyhow!(
                "Recursion is no more supported in this version, see https://github.com/zink-lang/zink/issues/248"
            )
            .into());
        }

        tracing::trace!("call internal function: index={index}");
        let (params, results) = self.env.funcs.get(&index).unwrap_or(&(0, 0));

        // TODO: adapat the case that the params is larger than 0xff (#247)
        //
        // 1. record the program counter of the end of this expression
        // call and pass it to the callee function.
        //
        // [ ..,
        //   <PC>,
        //   params[SWAP], params[PUSH, SLOT, MSTORE],
        //   PUSH, PC, JUMP, <JUMPDEST>
        // ]
        // <- selfparams[PUSH, OFFSET, CALLDATALOAD]
        //
        // 2. move PC before the params in stack
        self.table
            .offset(self.masm.pc_offset(), 5 + 4 * (*params as u16));
        self.masm.increment_sp(1)?;

        // Stack
        // =====
        //
        // from  [ <PARAMS>, PC ]
        // to    [ PC, <PARAMS> ]
        self.masm.shift_stack(*params as u8, true)?;

        // Call an internal function.
        //
        // 1. store params in memory
        // 2. register the call index to the jump table.
        let reserved = self.env.slots.get(&index).unwrap_or(&0);
        for i in (0..*params).rev() {
            tracing::trace!("storing local at {} for function {index}", i + reserved);
            self.masm.push(&((i + reserved) * 0x20).to_ls_bytes())?;
            self.masm._mstore()?;
        }

        // TODO: support same pc different jumps. (#160)
        self.table.call(self.masm.pc_offset(), index);

        // jump to the callee function
        self.masm._jump()?;
        self.masm._jumpdest()?;

        // Stack: [ , ..results ]
        self.masm.increment_sp(*results as u8)?;
        Ok(())
    }

    /// Call imported functions
    fn call_imported(&mut self, index: u32) -> Result<()> {
        // call an imported function.
        //
        // register the imported function index to the jump table.
        let func = *self
            .env
            .imports
            .get(&index)
            .ok_or(Error::ImportedFuncNotFound(index))?;

        tracing::trace!("call imported function, index={index}, func={func:?}");
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
                tracing::error!("unsupported host function {func:?}");
                Err(Error::UnsupportedHostFunc(func))
            }
        }
    }
}
