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
[workspace]

[dependencies]
anyhow = "1.0"
zint = { path = "../../../../zint" }
zink = { path = "../../../../../zink",version ="0.1.12",  features = ["abi-import"] } 
hex = "0.4"

[features]

abi-import = ["zink/abi-import"]
"#;
    fs::write(ztests_dir.join("Cargo.toml"), cargo_toml)?;

    fs::create_dir(ztests_dir.join("src"))?;
    let main_rs = r#"
use zink::primitives::U256;
use zint::{{Contract, EVM}};

#[cfg(feature = "abi-import")]
use zink::import;

fn main() {{
    println!("Run `cargo zint run` to execute tests");
}}

#[cfg(test)]
mod tests {
  
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_set_get() -> Result<()> {{
        #[cfg(not(feature = "abi-import"))]
        panic!("Test requires abi-import feature; run with `--features abi-import`");

        #[cfg(feature = "abi-import")]
        {
            import!("../../out/Storage.sol/Storage.json", "Storage" );


            let caller = hex::decode("be862ad9abfe6f22bcb087716c7d89a26051f74c")?;
            let mut caller_array = [0; 20];
            caller_array.copy_from_slice(&caller);

            let mut contract = Contract::search("Storage")?.compile()?;
            let mut evm = EVM::default().commit(true).caller(caller_array);
            let info = evm.deploy(&contract.bytecode()?)?;
            let address = info.address;

            let storage = Storage::new(address);
            let initial_value = storage.get_value()?;
            // println!("initial_value: {{:?}}", U256::from(initial_value));
            assert_eq!(initial_value, U256::from(0), "Initial value should be 0");

            storage.set_value(U256::from(42))?;
            let new_value = storage.get_value()?;
            println!("new_value: {{:?}}",  U256::from(new_value));
            assert_eq!(new_value, U256::from(42), "Value should be updated to 42");

            Ok(())
        }
    }}
}

"#;
    fs::write(ztests_dir.join("src/main.rs"), main_rs)?;

    println!("Created Zink testing crate at {:?}", ztests_dir);
    println!("Foundry output directory: {:?}", out_dir);
    Ok(())
}
fn run_zink_tests() -> Result<()> {
    let ztests_dir = PathBuf::from("ztests");
    if !ztests_dir.exists() {
        return Err(anyhow!("ztests directory not found; run `cargo zint new` first"));
    }

    let status = std::process::Command::new("cargo")
        .args(&["nextest", "run", "--manifest-path", "ztests/Cargo.toml", "--features", "abi-import"])
        .status()
        .map_err(|e| anyhow!("Failed to run tests: {}", e))?;

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