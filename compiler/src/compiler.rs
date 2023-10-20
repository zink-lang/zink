//! Zink compiler

use crate::{parser::Parser, Error, Result};
use wasmparser::{FuncValidator, FunctionBody, ValidatorResources, WasmModuleResources};
use zingen::{
    Buffer, CodeGen, DataSet, Dispatcher, Exports, Functions, Imports, JumpTable, BUFFER_LIMIT,
};

/// Zink Compiler
#[derive(Default)]
pub struct Compiler {
    buffer: Buffer,
    table: JumpTable,
}

impl Compiler {
    /// Compile wasm module to evm bytecode.
    pub fn compile(mut self, wasm: &[u8]) -> Result<Buffer> {
        let Parser {
            imports,
            data,
            mut funcs,
            exports,
        } = Parser::try_from(wasm)?;
        tracing::trace!("imports: {:?}", imports);
        tracing::trace!("data: {:?}", data);
        tracing::trace!("exports: {:?}", exports);

        let selectors = funcs.drain_selectors(&exports);
        self.compile_dispatcher(data.clone(), exports, imports.clone(), selectors)?;

        for func in funcs.into_funcs() {
            self.compile_func(data.clone(), imports.clone(), func.validator, func.body)?;
        }

        self.finish()
    }

    /// Compile EVM dispatcher.
    pub fn compile_dispatcher(
        &mut self,
        data: DataSet,
        exports: Exports,
        imports: Imports,
        selectors: Functions,
    ) -> Result<()> {
        let mut dispatcher = Dispatcher::default();
        dispatcher.data(data).exports(exports).imports(imports);

        let buffer = dispatcher.finish(selectors, &mut self.table)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(self.buffer.len()));
        }

        Ok(())
    }

    /// Compile WASM function.
    pub fn compile_func(
        &mut self,
        dataset: DataSet,
        imports: Imports,
        mut validator: FuncValidator<ValidatorResources>,
        body: FunctionBody,
    ) -> Result<()> {
        let func_index = validator.index();
        let sig = validator
            .resources()
            .type_of_function(func_index)
            .ok_or(Error::InvalidFunctionSignature)?
            .clone();

        tracing::debug!("compile function {}: {:?}", func_index, sig);

        let is_main = func_index - (imports.len() as u32) == 0;
        let mut codegen = CodeGen::new(sig, dataset, imports, is_main)?;
        let mut locals_reader = body.get_locals_reader()?;
        let mut ops_reader = body.get_operators_reader()?;

        codegen.emit_locals(&mut locals_reader, &mut validator)?;
        codegen.emit_operators(&mut ops_reader, &mut validator)?;

        self.emit_buffer(func_index, codegen)?;
        Ok(())
    }

    /// Emit buffer to the inner buffer.
    fn emit_buffer(&mut self, func_index: u32, codegen: CodeGen) -> Result<()> {
        let buffer = codegen.finish(&mut self.table, self.buffer.len() as u16)?;
        self.table
            .call_offset(func_index, self.buffer.len() as u16)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(buffer.len()));
        }

        Ok(())
    }

    /// Finish compilation.
    pub fn finish(mut self) -> Result<Buffer> {
        tracing::trace!("buffer length {:x}", self.buffer.len());
        self.table.code_offset(self.buffer.len() as u16);
        self.table.relocate(&mut self.buffer)?;

        Ok(self.buffer)
    }
}
