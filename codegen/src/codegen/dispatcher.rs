//! Code generator for EVM dispatcher.

use crate::{
    wasm::{self, Env, Functions, ToLSBytes},
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
        const RETURN_OFFSET: u8 = 0;
        const RETURN_SIZE: u8 = 32;

        let abi = self.env.load_abi(selector)?;
        self.abi.push(abi.clone());

        let selector_bytes = abi.selector();
        tracing::debug!(
            "Emitting selector {:?} for function: {}",
            selector_bytes,
            abi.signature(),
        );

        // Compare selectors.
        self.asm.push(&selector_bytes)?; // Stack: [selector, selector_bytes]
        self.asm._eq()?; // Stack: [result]

        // Conditional jump to function.
        let func = self.env.query_func(&abi.name)?;
        self.table.call(self.asm.pc(), func);
        self.asm._jumpi()?; // Jump to func if result != 0

        // Skip to next selector or stop.
        if last {
            self.asm._stop()?;
        } else {
            // Drop result of failed selector match
            self.asm._pop()?;
        }

        // Function return handling.
        let has_return = self
            .funcs
            .get(&func)
            .map(|ty| !ty.results().is_empty())
            .unwrap_or(false);
        if has_return {
            self.asm._jumpdest()?;
            self.asm.push(&RETURN_OFFSET.to_ls_bytes())?;
            self.asm._mstore()?;
            self.asm.push(&RETURN_SIZE.to_ls_bytes())?;
            self.asm.push(&RETURN_OFFSET.to_ls_bytes())?;
            self.asm._return()?;
        }

        Ok(())
    }
}
