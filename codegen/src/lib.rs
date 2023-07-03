//! Code generation library for zink.

pub use result::{Error, Result};
use wasmtime_environ::{
    wasmparser::{Parser, Validator},
    ModuleEnvironment, ModuleTranslation, Tunables,
};

mod result;

/// The code generation abstraction.
pub struct CodeGen;

impl CodeGen {
    /// Load webassembly binary and perform translation.
    pub fn load(wasm: &[u8]) -> Result<ModuleTranslation> {
        let parser = Parser::new(0);
        let mut types = Default::default();
        let tunables = Tunables::default();
        let mut validator = Validator::new();

        ModuleEnvironment::new(&tunables, &mut validator, &mut types)
            .translate(parser, wasm)
            .map_err(Into::into)
    }
}
