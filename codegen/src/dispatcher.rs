//! Code generator for EVM dispatcher.

use crate::{
    code::ExtFunc, DataSet, Error, Exports, Imports, JumpTable, MacroAssembler, Result, ToLSBytes,
};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use wasmparser::{
    FuncType, FuncValidator, FunctionBody, Operator, ValidatorResources, WasmModuleResources,
};
use zabi::Abi;

/// Function with validator.
pub struct Function<'f> {
    /// Function validator.
    pub validator: FuncValidator<ValidatorResources>,
    /// Function body.
    pub body: FunctionBody<'f>,
}

impl Function<'_> {
    /// Get function index.
    pub fn index(&self) -> u32 {
        self.validator.index()
    }

    /// Get the function signature.
    pub fn sig(&self) -> Result<FuncType> {
        let func_index = self.validator.index();
        let sig = self
            .validator
            .resources()
            .type_of_function(func_index)
            .ok_or(Error::InvalidFunctionSignature)?
            .clone();

        Ok(sig)
    }
}

/// Functions with indexes.
#[derive(Default)]
pub struct Functions<'f>(BTreeMap<u32, Function<'f>>);

impl<'f> Deref for Functions<'f> {
    type Target = BTreeMap<u32, Function<'f>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'f> DerefMut for Functions<'f> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'f> Functions<'f> {
    /// Add function to the list.
    pub fn add(
        &mut self,
        validator: FuncValidator<ValidatorResources>,
        function: FunctionBody<'f>,
    ) {
        self.0.insert(
            validator.index(),
            Function {
                validator,
                body: function,
            },
        );
    }

    /// Remove all selector functions
    pub fn drain_selectors(&mut self, exports: &Exports) -> Self {
        let mut functions = Self::default();

        for index in exports.selectors() {
            if let Some(function) = self.0.remove(&index) {
                functions.0.insert(index, function);
            }
        }

        functions
    }

    /// Get all functions
    pub fn into_funcs(self) -> Vec<Function<'f>> {
        self.0.into_values().collect()
    }
}

/// Code generator for EVM dispatcher.
pub struct Dispatcher<'d> {
    /// Code buffer
    pub asm: MacroAssembler,
    /// Module exports
    pub exports: Exports,
    /// Module functions
    pub funcs: &'d Functions<'d>,
    /// Module imports
    pub imports: Imports,
    /// Module data
    pub data: DataSet,
    /// Jump table
    pub table: JumpTable,
}

impl<'d> Dispatcher<'d> {
    /// Create dispatcher with functions.
    pub fn new(funcs: &'d Functions<'d>) -> Self {
        Self {
            asm: Default::default(),
            exports: Default::default(),
            funcs,
            imports: Default::default(),
            data: Default::default(),
            table: Default::default(),
        }
    }

    /// Set exports for the dispatcher.
    pub fn exports(&mut self, exports: Exports) -> &mut Self {
        self.exports = exports;
        self
    }

    /// Set imports for the dispatcher.
    pub fn imports(&mut self, imports: Imports) -> &mut Self {
        self.imports = imports;
        self
    }

    /// Set data for the dispatcher.
    pub fn data(&mut self, data: DataSet) -> &mut Self {
        self.data = data;
        self
    }

    /// Query exported function from selector.
    fn query_func(&self, name: &str) -> Result<u32> {
        for (index, export) in self.exports.iter() {
            if export.name == name {
                return Ok(*index);
            }
        }

        Err(Error::FuncNotImported(name.into()))
    }

    /// Load function ABI.
    fn load_abi(&mut self, selector: &Function<'_>) -> Result<Abi> {
        let mut reader = selector.body.get_operators_reader()?;

        // Get data offset.
        let Operator::I32Const { value: offset } = reader.read()? else {
            return Err(Error::InvalidSelector);
        };

        // Get data length.
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

        if !self.imports.is_emit_abi(index) {
            return Err(Error::FuncNotImported("emit_abi".into()));
        }

        Abi::from_hex_bytes(&self.data.load(offset, length as usize)?).map_err(Into::into)
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

        *self.asm = asm;
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
                // [ callee, ret, selector ] -> [ selector, callee, ret ]
                self.asm.shift_stack(2, false)?;
                // [ selector, callee, ret ] -> [ callee, ret ]
                self.asm._drop()?;
            } else {
                self.asm._swap1()?;
            }

            if len > 0 {
                // last: [ callee, ret ] -> [ param * len, ret, callee ]
                // !last: [ callee, ret ] -> [ param * len, callee, ret ]
                for p in (0..len).rev() {
                    let offset = 4 + p * 32;
                    self.asm.push(&offset.to_ls_bytes())?;
                    self.asm._calldataload()?;
                }

                // [ param * len, callee, ret ] -> [ ret, param * len, callee ]
                self.asm.shift_stack(len + 1, false)?;
                // [ ret, param * len, callee ] -> [ callee, ret, param * len ]
                self.asm.shift_stack(len + 1, false)?;
            }

            self.asm._jump()?;
        }

        let bytecode = {
            let jumpdest = vec![0x5b];
            let ret = self.asm.buffer()[asm.buffer().len()..].to_vec();
            [jumpdest, ret].concat()
        };
        *self.asm = asm;
        let ret = ExtFunc {
            bytecode,
            stack_in: len,
            stack_out: 1,
        };
        self.table.ext(self.asm.pc_offset(), ret);
        Ok(true)
    }

    /// Emit selector to buffer.
    fn emit_selector(&mut self, selector: &Function<'_>, last: bool) -> Result<()> {
        let abi = self.load_abi(selector)?;
        let selector_bytes = abi.selector();

        tracing::debug!(
            "Emitting selector {:?} for function: {}",
            selector_bytes,
            abi.signature()
        );

        let func = self.query_func(&abi.name)?;
        let sig = self
            .funcs
            .get(&func)
            .ok_or(Error::FuncNotFound(func))?
            .sig()?;

        // Jump to the end of the current function.
        //
        // TODO: detect the bytes of the position. (#157)
        self.ext_return(&sig)?;

        // Prepare the `PC` of the callee function.
        {
            // TODO: remove this (#160)
            self.asm._jumpdest()?;
            self.asm.increment_sp(1)?;
            self.table.call(self.asm.pc_offset(), func);
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
            self.asm.shift_stack(2, false)?;
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

    /// Emit compiled code to the given buffer.
    pub fn finish(mut self, selectors: Functions<'_>, table: &mut JumpTable) -> Result<Vec<u8>> {
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

        table.merge(self.table, 0)?;
        Ok(self.asm.buffer().into())
    }
}
