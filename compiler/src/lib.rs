//! Zink compiler.

pub use crate::result::{Error, Result};
use wasmparser::{Parser, ValidPayload, Validator, WasmModuleResources};
use zingen::CodeGen;

pub mod result;

/// Zink Compiler
#[derive(Default)]
pub struct Compiler;

impl Compiler {
    /// Compile wasm moudle to evm bytecode.
    pub fn compile(wasm: &[u8]) -> Result<Vec<u8>> {
        let mut validator = Validator::new();

        let mut bin = Vec::new();
        let parser = Parser::new(0);
        for payload in parser.parse_all(wasm) {
            let payload = validator.payload(&payload?)?;
            if let ValidPayload::Func(to_validator, body) = payload {
                let mut func_validator = to_validator.into_validator(Default::default());
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
                bin.extend_from_slice(codegen.buffer());
            }
        }

        Ok(bin)
    }
}
