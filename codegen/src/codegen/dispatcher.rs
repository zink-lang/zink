//! Code generator for EVM dispatcher.

use std::collections::BTreeMap;

use crate::{
    codegen::code::ExtFunc,
    wasm::{self, Env, Functions, ToLSBytes},
    Error, JumpTable, MacroAssembler, Result,
};
use wasmparser::{FuncType, Operator};
use zabi::Abi;

/// Code generator for EVM dispatcher.
pub struct Dispatcher {
    /// ABI for the current function
    pub abi: Vec<Abi>,
    /// Code buffer
    pub asm: MacroAssembler,
    /// WASM environment
    pub env: Env,
    /// Module functions
    pub funcs: BTreeMap<u32, FuncType>,
    /// Jump table
    pub table: JumpTable,
}

impl Dispatcher {
    /// Create dispatcher with functions.
    pub fn new(env: Env, funcs: &Functions<'_>) -> Result<Self> {
        let funcs = funcs
            .values()
            .map(|func| Ok((func.index(), func.sig()?)))
            .collect::<Result<_>>()?;

        Ok(Self {
            abi: Default::default(),
            asm: Default::default(),
            env,
            funcs,
            table: Default::default(),
        })
    }

    /// Emit compiled code to the given buffer.
    pub fn finish(&mut self, selectors: Functions<'_>, table: &mut JumpTable) -> Result<Vec<u8>> {
        if selectors.is_empty() {
            return Err(Error::SelectorNotFound);
        }

        self.asm._push0()?;
        self.asm._calldataload()?;
        self.asm.push(&[0xe0])?;
        self.asm._shr()?;

        let mut len = selectors.len();
        for (_, func) in selectors.iter() {
            self.emit_selector(func, len == 1)?;
            len -= 1;
        }

        table.merge(self.table.clone(), 0)?;
        Ok(self.asm.buffer().into())
    }

    /// Query exported function from selector.
    fn query_func(&self, name: &str) -> Result<u32> {
        for (index, export) in self.env.exports.iter() {
            if export == name {
                return Ok(*index);
            }
        }

        Err(Error::FuncNotImported(name.into()))
    }

    /// Load function ABI.
    fn load_abi(&mut self, selector: &wasm::Function<'_>) -> Result<Abi> {
        let mut reader = selector.body.get_operators_reader()?;

        let Operator::I32Const { value: offset } = reader.read()? else {
            return Err(Error::InvalidSelector);
        };
        let Operator::I32Const { value: length } = reader.read()? else {
            return Err(Error::InvalidSelector);
        };

        // Validate zinkc helper `emit_abi`
        let Operator::Call {
            function_index: index,
        } = reader.read()?
        else {
            return Err(Error::InvalidSelector);
        };

        if !self.env.imports.is_emit_abi(index) {
            return Err(Error::FuncNotImported("emit_abi".into()));
        }

        let abi = self.env.data.load(offset, length as usize)?;
        Abi::from_hex(String::from_utf8_lossy(&abi)).map_err(Into::into)
    }

    /// Emit return of ext function.
    fn ext_return(&mut self, sig: &FuncType) -> Result<()> {
        self.asm.increment_sp(1)?;
        let asm = self.asm.clone();

        {
            self.asm.main_return(sig.results())?;
        }

        let bytecode = {
            let jumpdest = vec![0x5b];
            let ret = self.asm.buffer()[asm.buffer().len()..].to_vec();
            [jumpdest, ret].concat()
        };

        self.asm = asm;
        let ret = ExtFunc {
            bytecode,
            stack_in: 0,
            stack_out: 0,
        };
        self.table.ext(self.asm.pc_offset(), ret);
        Ok(())
    }

    // Process to the selected function.
    //
    // 1. drop selector.
    // 2. load calldata to stack.
    // 3. jump to the callee function.
    fn process(&mut self, len: usize, last: bool) -> Result<bool> {
        let len = len as u8;
        if last && len == 0 {
            return Ok(false);
        }

        self.asm.increment_sp(1)?;
        let asm = self.asm.clone();
        {
            if !last {
                // TODO: check the safety of this.
                //
                // [ ret, callee, selector ] -> [ selector, ret, callee ]
                self.asm.shift_stack(2, false)?;
                // [ selector, ret, callee ] -> [ ret, callee ]
                self.asm._drop()?;
            } else {
                self.asm._swap1()?;
            }

            if len > 0 {
                // [ ret, callee ] -> [ param * len, ret, callee ]
                for p in (0..len).rev() {
                    let offset = 4 + p * 32;
                    self.asm.push(&offset.to_ls_bytes())?;
                    self.asm._calldataload()?;
                }

                // [ param * len, ret, callee ] -> [ ret, param * len, callee ]
                self.asm.shift_stack(len, false)?;
                // [ ret, param * len, callee ] -> [ callee, ret, param * len ]
                self.asm.shift_stack(len + 1, false)?;
            } else {
                self.asm._swap1()?;
            }

            self.asm._jump()?;
        }

        let bytecode = {
            let jumpdest = vec![0x5b];
            let ret = self.asm.buffer()[asm.buffer().len()..].to_vec();
            [jumpdest, ret].concat()
        };
        self.asm = asm;
        let ret = ExtFunc {
            bytecode,
            stack_in: len,
            stack_out: 1,
        };
        self.table.ext(self.asm.pc_offset(), ret);
        Ok(true)
    }

    /// Emit selector to buffer.
    fn emit_selector(&mut self, selector: &wasm::Function<'_>, last: bool) -> Result<()> {
        let abi = self.load_abi(selector)?;

        // TODO: refactor this. (#206)
        self.abi.push(abi.clone());

        let selector_bytes = abi.selector();

        tracing::trace!(
            "Emitting selector {:?} for function: {}",
            selector_bytes,
            abi.signature(),
        );

        let func = self.query_func(&abi.name)?;
        let sig = self
            .funcs
            .get(&func)
            .ok_or(Error::FuncNotFound(func))?
            .clone();

        // TODO: optimize this on parameter length (#165)
        {
            // Prepare the `PC` of the callee function.
            //
            // TODO: remove this (#160)
            {
                self.asm.increment_sp(1)?;
                self.table.call(self.asm.pc_offset(), func);
                self.asm._jumpdest()?;
            }

            // Jump to the end of the current function.
            //
            // TODO: detect the bytes of the position. (#157)
            self.ext_return(&sig)?;
        }

        if last {
            self.asm._swap2()?;
        } else {
            self.asm._dup3()?;
        }

        self.asm.push(&selector_bytes)?;
        self.asm._eq()?;
        let processed = self.process(sig.params().len(), last)?;
        if last && !processed {
            self.asm._swap1()?;
        }
        self.asm._jumpi()?;

        if !last {
            // drop the PC of the previous callee function.
            self.asm._drop()?;
            // drop the PC of the previous callee function preprocessor.
            self.asm._drop()?;
        }

        Ok(())
    }
}
