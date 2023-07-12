//! Zink compiler

use crate::{buffer::Buffer, Error, Result};
use wasmparser::{
    FuncToValidate, FunctionBody, Parser, ValidPayload, Validator, ValidatorResources,
    WasmModuleResources,
};
use zingen::CodeGen;

/// Zink Compiler
#[derive(Default)]
pub struct Compiler {
    inner: Vec<Buffer>,
}

impl Compiler {
    /// Compile wasm moudle to evm bytecode.
    pub fn compile(mut self, wasm: &[u8]) -> Result<Vec<u8>> {
        let mut validator = Validator::new();
        for payload in Parser::new(0).parse_all(wasm) {
            let payload = validator.payload(&payload?)?;
            if let ValidPayload::Func(to_validator, body) = payload {
                self.compile_func(to_validator, body)?;
            }
        }

        Ok(self.finish())
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

        self.emit_buffer(codegen.finish());
        Ok(())
    }

    /// Finish compilation.
    pub fn finish(mut self) -> Vec<u8> {
        self.patch();

        self.inner
            .into_iter()
            .map(|b| b.buffer().into())
            .collect::<Vec<Vec<u8>>>()
            .into_iter()
            .flatten()
            .collect()
    }

    /// Emit buffer to the inner buffer.
    fn emit_buffer(&mut self, buffer: Buffer) {
        self.inner.push(buffer);
    }

    /// Patch labels to fix jump offsets.
    fn patch(&mut self) {}
}
