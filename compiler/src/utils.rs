//! Zink compiler utils
#![cfg(feature = "utils")]

use std::path::Path;

#[cfg(feature = "wasm-opt")]
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

#[cfg(not(feature = "wasm-opt"))]
/// Run wasm-opt on the given WASM file.
pub fn wasm_opt(wasm: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    use std::process::{Command, Stdio};

    let [input, output] = [
        wasm.as_ref().to_string_lossy(),
        output.as_ref().to_string_lossy(),
    ];
    let output = Command::new("wasm-opt")
        .args([
            input.as_ref(),
            "--mvp-features",
            "-O4",
            "-s",
            "2",
            "--converge",
            "-o",
            output.as_ref(),
        ])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;

    if !output.stderr.is_empty() {
        return Err(anyhow::anyhow!(
            "{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
