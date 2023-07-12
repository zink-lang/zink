//! Zink compiler

use crate::{Error, Result};
use wasmparser::{
    FuncToValidate, FunctionBody, Parser, ValidPayload, Validator, ValidatorResources,
    WasmModuleResources,
};
use zingen::{Buffer, CodeGen, JumpTable, BUFFER_LIMIT};

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
        for payload in Parser::new(0).parse_all(wasm) {
            let payload = validator.payload(&payload?)?;
            if let ValidPayload::Func(to_validator, body) = payload {
                self.compile_func(to_validator, body)?;
            }
        }

        self.finish()
    }

    /// Compile WASM function.
    pub fn compile_func(
        &mut self,
        validator: FuncToValidate<ValidatorResources>,
        body: FunctionBody,
    ) -> Result<()> {
        let mut func_validator = validator.into_validator(Default::default());
        let sig = func_validator
            .resources()
            .type_of_function(0)
            // TODO: Add backtrace here for the function index. (#21)
            .ok_or(Error::InvalidFunctionSignature)?
            .clone();

        let mut codegen = CodeGen::new(sig);
        let mut locals_reader = body.get_locals_reader()?;
        let mut ops_reader = body.get_operators_reader()?;

        codegen.emit_locals(&mut locals_reader, &mut func_validator)?;
        codegen.emit_operators(&mut ops_reader, &mut func_validator)?;

        self.emit_buffer(codegen)?;
        Ok(())
    }

    /// Finish compilation.
    pub fn finish(mut self) -> Result<Buffer> {
        self.table.patch(&mut self.buffer)?;

        Ok(self.buffer)
    }

    /// Emit buffer to the inner buffer.
    fn emit_buffer(&mut self, codegen: CodeGen) -> Result<()> {
        let buffer = codegen.finish(&mut self.table, self.buffer.len() as u16)?;
        if !self.buffer.is_empty() {
            self.buffer.push(0x5b);
        }

        self.buffer.extend_from_slice(&buffer);
        if buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(buffer.len()));
        }

        Ok(())
    }
}
