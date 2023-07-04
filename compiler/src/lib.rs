//! Zink compiler.

pub use crate::{
    config::{Profile, Target},
    result::{Error, Result},
    wasm::WasmBuilder,
};
use anyhow::anyhow;
use cranelift_wasm::{DefinedFuncIndex, TypeConvert};
use wasmtime_environ::{
    wasmparser::{Parser, Validator},
    FunctionBodyData, ModuleEnvironment, ModuleTranslation, Tunables,
};
use winch_codegen::TargetIsa;
use zgen::EvmIsa;

mod config;
pub mod result;
mod wasm;

/// Zink compiler.
///
/// TODO: support cranelift.
pub struct Zinkc;

impl Zinkc {
    /// Compile webassembly binary to evm bytecode.
    pub fn compile(wasm: &[u8]) -> Result<Vec<u8>> {
        let mut translation = Self::translate(wasm)?;
        let body_inputs = std::mem::take(&mut translation.function_body_inputs);

        Ok(body_inputs
            .into_iter()
            .map(|f| Self::compile_function_body(&translation, f.0, f.1))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    /// Translate webassembly module.
    fn translate(wasm: &[u8]) -> Result<ModuleTranslation> {
        let parser = Parser::new(0);
        let mut types = Default::default();
        let tunables = Tunables::default();
        let mut validator = Validator::new();

        ModuleEnvironment::new(&tunables, &mut validator, &mut types)
            .translate(parser, wasm)
            .map_err(Into::into)
    }

    fn compile_function_body(
        translation: &ModuleTranslation,
        index: DefinedFuncIndex,
        body: FunctionBodyData<'_>,
    ) -> Result<Vec<u8>> {
        let index = translation.module.func_index(index);
        let FunctionBodyData { body, validator } = body;

        // get function signature
        let sig = {
            let ty = translation
                .get_types()
                .function_at(index.as_u32())
                .ok_or(anyhow!("function type at index {:?}", index.as_u32()))?;

            translation.module.convert_func_type(ty)
        };

        // compile function.
        Ok(EvmIsa
            .compile_function(
                &sig,
                &body,
                translation,
                &mut validator.into_validator(Default::default()),
            )?
            .data()
            .to_vec())
    }
}
