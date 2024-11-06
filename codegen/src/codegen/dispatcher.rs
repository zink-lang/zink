//! Code generator for EVM dispatcher.

use crate::{
    wasm::{self, Env, Functions},
    JumpTable, MacroAssembler, Result,
};
use std::collections::BTreeMap;
use wasmparser::FuncType;
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
            return Ok(Default::default());
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

    /// Emit selector to buffer.
    fn emit_selector(&mut self, selector: &wasm::Function<'_>, last: bool) -> Result<()> {
        let abi = self.env.load_abi(selector)?;
        self.abi.push(abi.clone());

        let selector_bytes = abi.selector();
        tracing::debug!(
            "Emitting selector {:?} for function: {}",
            selector_bytes,
            abi.signature(),
        );

        let func = self.env.query_func(&abi.name)?;
        self.asm.increment_sp(1)?;

        // Prepare the `PC` of the callee function.
        self.table.call(self.asm.pc_offset(), func);

        if last {
            self.asm._swap1()?;
        } else {
            self.asm._dup2()?;
        }

        self.asm.push(&selector_bytes)?;
        self.asm._eq()?;
        self.asm._swap1()?;
        self.asm._jumpi()?;

        Ok(())
    }
}
