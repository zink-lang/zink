//! Zink compiler

use crate::{parser::Parser, Error, Result};
use wasmparser::{FuncToValidate, FunctionBody, ValidatorResources, WasmModuleResources};
use zingen::{Buffer, CodeGen, DataSet, Imports, JumpTable, BUFFER_LIMIT};

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
            funcs,
        } = Parser::try_from(wasm)?;

        for (index, (func, body)) in funcs.into_iter() {
            self.compile_func(index, data.clone(), imports.clone(), func, body)?;
        }

        self.finish()
    }

    /// Compile WASM function.
    pub fn compile_func(
        &mut self,
        func_index: u32,
        dataset: DataSet,
        imports: Imports,
        validator: FuncToValidate<ValidatorResources>,
        body: FunctionBody,
    ) -> Result<()> {
        let mut func_validator = validator.into_validator(Default::default());
        let sig = func_validator
            .resources()
            // NOTE: the functions list is [ [imports] [funcs] ] so
            // here we need to add the length of imports to get the
            // correct index.
            .type_of_function(func_index + imports.len() as u32)
            // TODO: Add backtrace here for the function index. (#21)
            .ok_or(Error::InvalidFunctionSignature)?
            .clone();

        tracing::debug!("compile function {}: {:?}", func_index, sig);

        let is_main = func_index == 0;
        let mut codegen = CodeGen::new(sig, dataset, imports, is_main)?;
        let mut locals_reader = body.get_locals_reader()?;
        let mut ops_reader = body.get_operators_reader()?;

        codegen.emit_locals(&mut locals_reader, &mut func_validator)?;
        codegen.emit_operators(&mut ops_reader, &mut func_validator)?;

        self.emit_buffer(func_index, codegen)?;
        Ok(())
    }

    /// Finish compilation.
    pub fn finish(mut self) -> Result<Buffer> {
        tracing::trace!("buffer length {:x}", self.buffer.len());
        self.table.code_offset(self.buffer.len() as u16);
        self.table.relocate(&mut self.buffer)?;

        Ok(self.buffer)
    }

    /// Emit buffer to the inner buffer.
    fn emit_buffer(&mut self, func_index: u32, codegen: CodeGen) -> Result<()> {
        let buffer = codegen.finish(&mut self.table, self.buffer.len() as u16)?;
        self.table
            .call_offset(func_index, self.buffer.len() as u16)?;
        self.buffer.extend_from_slice(&buffer);

        if buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(buffer.len()));
        }

        Ok(())
    }
}
