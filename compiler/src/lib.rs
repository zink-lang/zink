//! Zink compiler.

pub use crate::result::{Error, Result};
use wasmparser::{Parser, ValidPayload, Validator};
use zingen::CodeGen;

pub mod result;

/// Zink Compiler
pub struct Compiler;

impl Compiler {
    /// Compile wasm moudle to evm bytecode.
    pub fn compile(wasm: &[u8]) -> Vec<u8> {
        let mut validator = Validator::new();

        let mut bin = Vec::new();
        let parser = Parser::new(0);
        for payload in parser.parse_all(wasm) {
            let payload = validator.payload(&payload.unwrap()).unwrap();
            if let ValidPayload::Func(to_validator, body) = payload {
                let mut codegen = CodeGen::new();
                let mut func_validator = to_validator.into_validator(Default::default());
                // let mut locals_reader = body.get_locals_reader().unwrap();
                let mut ops_reader = body.get_operators_reader().unwrap();

                // let sig = func_validator
                //     .resources()
                //     .type_of_function(0)
                //     .unwrap()
                //     .clone();

                // codegen
                //     .emit_locals(sig, &mut locals_reader, &mut func_validator)
                //     .unwrap();

                codegen
                    .emit_operators(&mut ops_reader, &mut func_validator)
                    .unwrap();

                bin.extend_from_slice(codegen.buffer());
            }
        }

        bin
    }
}
