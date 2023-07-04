//! Ethereum instruction set architecture.
#![allow(unused)]

use anyhow::Result;
use cranelift_codegen::{
    settings::{Flags, Value},
    Final, MachBufferFinalized, TextSectionBuilder,
};
use cranelift_wasm::{
    wasmparser::{FuncValidator, FunctionBody, ValidatorResources},
    WasmFuncType,
};
use target_lexicon::Triple;
use wasmtime_environ::ModuleTranslation;
use winch_codegen::{isa::TargetIsa, TrampolineKind};

/// Ethereum instruction set architecture.
pub struct EvmIsa;

impl TargetIsa for EvmIsa {
    fn name(&self) -> &'static str {
        "evm"
    }

    fn triple(&self) -> &Triple {
        todo!("Introduce evm triple")
    }

    fn flags(&self) -> &Flags {
        todo!("configure evm instructions")
    }

    fn isa_flags(&self) -> Vec<Value> {
        todo!("based on flags")
    }

    fn compile_function(
        &self,
        sig: &WasmFuncType,
        body: &FunctionBody,
        translation: &ModuleTranslation,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<cranelift_codegen::MachBufferFinalized<cranelift_codegen::Final>> {
        todo!("after implementing evm instructions")
    }

    fn text_section_builder(&self, num_labeled_funcs: usize) -> Box<dyn TextSectionBuilder> {
        todo!("after implementing evm instructions")
    }

    fn function_alignment(&self) -> u32 {
        todo!("research on function alignment in EVM")
    }

    fn compile_trampoline(
        &self,
        ty: &WasmFuncType,
        kind: TrampolineKind,
    ) -> Result<MachBufferFinalized<Final>> {
        unimplemented!("no need to implement this")
    }
}
