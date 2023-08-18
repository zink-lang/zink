//! Zink compiler

use crate::{Error, Result};
use wasmparser::{
    FuncToValidate, FunctionBody, Import, Parser, Payload, TypeRef, ValidPayload, Validator,
    ValidatorResources, WasmModuleResources,
};
use zingen::{Buffer, CodeGen, Func, Imports, JumpTable, BUFFER_LIMIT};

/// Zink Compiler
#[derive(Default)]
pub struct Compiler {
    buffer: Buffer,
    table: JumpTable,
}

impl Compiler {
    /// Compile wasm moudle to evm bytecode.
    pub fn compile(mut self, wasm: &[u8]) -> Result<Buffer> {
        let mut validator = Validator::new();
        let mut func_index = 0;
        let mut imports = Imports::default();

        // Compile functions.
        for payload in Parser::new(0).parse_all(wasm) {
            // Get imported functions
            //
            // NOTE: this is safe here since the import section is
            // ahead of the function section after the optimization
            // of wasm-opt.
            if let Ok(Payload::ImportSection(reader)) = &payload {
                let mut iter = reader.clone().into_iter();
                while let Some(Ok(Import {
                    module,
                    name,
                    ty: TypeRef::Func(index),
                })) = iter.next()
                {
                    if let Ok(func) = Func::try_from((module, name)) {
                        tracing::debug!("imported function: {}::{}", module, name);
                        imports.insert(index, func);
                    }
                }

                validator.payload(&payload?)?;
                continue;
            }

            let payload = validator.payload(&payload?)?;
            if let ValidPayload::Func(to_validator, body) = payload {
                self.compile_func(func_index, imports.clone(), to_validator, body)?;
                func_index += 1;
            }
        }

        self.finish()
    }

    /// Compile WASM function.
    pub fn compile_func(
        &mut self,
        func_index: u32,
        imports: Imports,
        validator: FuncToValidate<ValidatorResources>,
        body: FunctionBody,
    ) -> Result<()> {
        let mut func_validator = validator.into_validator(Default::default());
        let sig = func_validator
            .resources()
            .type_of_function(func_index)
            // TODO: Add backtrace here for the function index. (#21)
            .ok_or(Error::InvalidFunctionSignature)?
            .clone();

        tracing::debug!("compile function {}: {:?}", func_index, sig);

        let is_main = func_index == 0;
        let mut codegen = CodeGen::new(sig, imports, is_main)?;
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
