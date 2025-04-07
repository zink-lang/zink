use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "cargo-zint")]
#[command(about = "Zink testing tool for Foundry projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Zink testing crate
    New,
    /// Run Zink tests
    Run,
}

#[derive(Deserialize)]
struct FoundryToml {
    profile: Option<Profile>,
}

#[derive(Deserialize)]
struct Profile {
    default: Option<DefaultProfile>,
}

#[derive(Deserialize)]
struct DefaultProfile {
    out: Option<String>,
}

fn find_foundry_out_dir() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;
    loop {
        let toml_path = current_dir.join("foundry.toml");
        if toml_path.exists() {
            let toml_content = fs::read_to_string(&toml_path)?;
            let foundry_toml: FoundryToml = toml::from_str(&toml_content)?;
            let out_dir = foundry_toml
                .profile
                .and_then(|p| p.default)
                .and_then(|d| d.out)
                .unwrap_or("out".to_string());
            return Ok(current_dir.join(out_dir));
        }
        if !current_dir.pop() {
            return Err(anyhow!("Could not find foundry.toml in any parent directory"));
        }
    }
}

fn create_zink_testing_crate(out_dir: &Path) -> Result<()> {
    let ztests_dir = PathBuf::from("ztests");
    if ztests_dir.exists() {
        return Err(anyhow!("ztests directory already exists"));
    }
    fs::create_dir(&ztests_dir)?;

    let cargo_toml = r#"
[package]
name = "ztests"
version = "0.1.0"
edition = "2021"
authors = ["Zink Team <team@zink-lang.org>"]
license = "MIT OR Apache-2.0"
homepage = "https://github.com/zink-lang/zink"
repository = "https://github.com/zink-lang/zink"

[dependencies]
anyhow = "1.0"
zint = { path = "../../" }
zink = { path = "../../../zink" }
smallvec = "1.13"
hex = "0.4"
"#;
    fs::write(ztests_dir.join("Cargo.toml"), cargo_toml)?;

    fs::create_dir(ztests_dir.join("src"))?;
    let main_rs = r#"
use zink::primitives::{Bytes32, U256};
use zint::{Contract, EVM};

fn main() {
    println!("Run `cargo zint` to execute tests");
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use smallvec::SmallVec;

    #[test]
    fn test_storage_set_get() -> Result<()> {
        let caller_bytes = hex::decode("be862ad9abfe6f22bcb087716c7d89a26051f74c")?;
        let mut caller = [0; 20];
        caller.copy_from_slice(&caller_bytes);

        let mut evm = EVM::default().commit(true).caller(caller);
        let mut contract = Contract::search("storage_test")?.compile()?;

        let value = U256::from(42);
        let value_bytes = value.to_be_bytes::<32>();

        let info = evm.deploy(&contract.bytecode()?)?;
        let address = info.address;

        let info = evm.calldata(&contract.encode(&[b"getValue()".to_vec()])?).call(address)?;
        assert_eq!(info.ret, vec![0u8; 32], "Getter should return default 0 initially");

        Ok(())
    }
}
"#;
    fs::write(ztests_dir.join("src/main.rs"), main_rs)?;

    let storage_test = r#"
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::primitives::{Bytes32, U256};

#[zink::external]
pub fn get_value() -> U256 {
    U256::from(0)
}

#[zink::external]
pub fn set_value(value: U256) {
    // Placeholder
}
"#;
    fs::write(ztests_dir.join("storage_test.zink"), storage_test)?;

    println!("Created Zink testing crate at {:?}", ztests_dir);
    println!("Foundry output directory: {:?}", out_dir);
    Ok(())
}

fn run_zink_tests() -> Result<()> {
    let ztests_dir = PathBuf::from("ztests");
    if !ztests_dir.exists() {
        return Err(anyhow!("ztests directory not found; run `cargo zint new` first"));
    }

    let zinkc_cmd = std::process::Command::new("zinkc")
        .args(&["ztests/storage_test.zink", "-o", "ztests/storage_test.bin"])
        .status()
        .map_err(|e| anyhow!("Failed to compile Zink test: {}", e))?;
    println!("zinkc exit status: {:?}", zinkc_cmd);

    let bin_path = PathBuf::from("ztests/storage_test.bin");
    if !bin_path.exists() {
        return Err(anyhow!("zinkc did not generate storage_test.bin"));
    }

    let bin_content = fs::read(&bin_path)?;
    if &bin_content[0..4] != &[0x00, 0x61, 0x73, 0x6d] {
        return Err(anyhow!("Compiled Zink test is not a valid WASM binary"));
    }

    let status = std::process::Command::new("cargo")
        .args(&["nextest", "run", "--manifest-path", "ztests/Cargo.toml"])
        .status()
        .map_err(|e| anyhow!("Failed to run tests (ensure cargo-nextest is installed): {}", e))?;

    if !status.success() {
        return Err(anyhow!("Tests failed"));
    }

    println!("Zink tests completed successfully");
    Ok(())
}
pub fn run(cli: Cli) -> Result<()> {
    let out_dir = find_foundry_out_dir()?;
    match cli.command {
        Commands::New => create_zink_testing_crate(&out_dir)?,
        Commands::Run => run_zink_tests()?,
    }
    Ok(())
}