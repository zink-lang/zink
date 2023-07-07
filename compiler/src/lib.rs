//! Zink compiler.

pub use crate::result::{Error, Result};

pub mod result;

#[cfg(test)]
mod tests {
    use wasmparser::{Parser, ValidPayload, Validator};
    use zingen::CodeGen;

    #[test]
    fn test_addition() {
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

        let mut validator = Validator::new();

        let parser = Parser::new(0);
        for payload in parser.parse_all(&wasm) {
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

                println!("{:x?}", codegen.buffer());
            }
        }
    }
}
