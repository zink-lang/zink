//! Zink compiler utils
#![cfg(feature = "utils")]

use std::path::Path;

/// Run wasm-opt on the given WASM file.
pub fn wasm_opt(input: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    ::wasm_opt::OptimizationOptions::new_opt_level_4()
        .shrink_level(::wasm_opt::ShrinkLevel::Level2)
        .debug_info(false)
        .mvp_features_only()
        .set_converge()
        .run(&input, &output)
        .map_err(Into::into)
}
