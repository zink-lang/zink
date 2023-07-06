//! Code generation library for zink.
#![deny(missing_docs)]

use wasmparser::{BinaryReader, FuncValidator, FunctionBody, ValidatorResources};

use crate::parser::ValidateThenVisit;
pub use crate::{
    asm::Assmbler,
    masm::MacroAssembler,
    result::{Error, Result},
};

mod asm;
mod context;
mod limits;
mod masm;
mod parser;
mod result;
mod stack;
mod visitor;

/// The code generation abstraction.
///
/// TODO: add codegen context for backtrace.
#[derive(Default)]
pub struct CodeGen {
    masm: MacroAssembler,
}

impl CodeGen {
    /// Create a new code generator.
    pub fn new() -> Self {
        Self {
            masm: MacroAssembler::default(),
        }
    }

    /// Get the generated code.
    pub fn buffer(&self) -> &[u8] {
        self.masm.buffer()
    }

    /// Emit function body
    pub fn emit_body<'a>(
        &mut self,
        body: &mut BinaryReader<'a>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        while !body.eof() {
            let offset = body.original_position();
            let _ = body.visit_operator(&mut ValidateThenVisit(validator.visitor(offset), self))?;
        }

        Ok(())
    }
}

#[test]
fn test_addition() {
    use wasmparser::ValidPayload;

    let wasm = wat::parse_str(
        r#"
(module
  (func (export "add") (param i32 i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
  )
)
"#,
    )
    .unwrap();

    let mut codegen = CodeGen::new();
    let mut validator = wasmparser::Validator::new();

    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(&wasm) {
        let payload = validator.payload(&payload.unwrap()).unwrap();
        if let ValidPayload::Func(to_validator, body) = payload {
            let mut val = to_validator.into_validator(Default::default());
            let mut reader = body.get_binary_reader();

            // val.finish(offset)
            codegen.emit_body(&mut reader, &mut val).unwrap();
        }
    }
}
