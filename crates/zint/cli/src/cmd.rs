use anyhow::{Context, Result};
use std::fs;
use zint::utils::{find_up, FoundryConfig};
use zint::Contract;

pub fn create_ztests_crate() -> Result<()> {
    // Find foundry.toml in the current directory or parent directories
    let foundry_toml_path = find_up("foundry.toml")?;
    let foundry_toml_content =
        fs::read_to_string(&foundry_toml_path).context("Failed to read foundry.toml")?;
    let foundry_config: FoundryConfig =
        toml::from_str(&foundry_toml_content).context("Failed to parse foundry.toml")?;

    // Determine the output directory (default to "out" if not specified)
    let out_dir = foundry_config
        .profile
        .default
        .out
        .unwrap_or_else(|| "out".to_string());
    let _out_path = foundry_toml_path.parent().unwrap().join(&out_dir);

    // Create ztests directory
    let ztests_path = foundry_toml_path.parent().unwrap().join("ztests");
    fs::create_dir_all(&ztests_path).context("Failed to create ztests directory")?;

    // Fetch the zink version from the environment variable set by zint-cli's build.rs
    let zink_version = std::env::var("ZINK_VERSION").unwrap_or_else(|_| "0.1.12".to_string());

    // Write ztests/Cargo.toml with workspace dependencies
let cargo_toml_content = format!(
        r#"
[package]
name = "ztests"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
zink = "{zink_version}"
zint = "^0.1"


[lib]
doctest = false

[features]
abi-import = ["zink/abi-import"]

[workspace]
"#,
        zink_version = zink_version
    );
    fs::write(ztests_path.join("Cargo.toml"), cargo_toml_content)
        .context("Failed to write ztests/Cargo.toml")?;

    // Create ztests/src directory
    let ztests_src_path = ztests_path.join("src");
    fs::create_dir_all(&ztests_src_path).context("Failed to create ztests/src directory")?;

    // Use find_foundry_outputs() to get the list of contract artifacts
    let outputs = Contract::find_foundry_outputs()?;
    if outputs.is_empty() {
        println!("No Foundry outputs found");
    }

    // Find all .json files in the out directory (compiled contract ABIs)
    let mut test_file_content = String::from(
        r#"#[cfg(test)]
mod tests {
    #[cfg(feature = "abi-import")]
    use zink::import;
    
    #[allow(unused_imports)]
    use zink::primitives::address::Address;
    #[allow(unused_imports)]
    use zink::primitives::u256::U256;

"#,
    );

    for (contract_name, abi_path, _bytecode) in outputs {
        let file_name = abi_path.file_name().unwrap().to_str().unwrap();
        let contract_struct_name = contract_name;

        // Parse the ABI to generate specific tests
        let abi_content = fs::read_to_string(&abi_path)?;
        let abi: serde_json::Value = match serde_json::from_str(&abi_content) {
            Ok(abi) => abi,
            Err(e) => {
                println!("Failed to parse ABI for {}: {}", file_name, e);
                continue;
            }
        };
        let abi_array = match abi["abi"].as_array() {
            Some(array) => array,
            None => {
                println!("No 'abi' field found in {}: {:?}", file_name, abi);
                continue;
            }
        };

        let mut test_body = String::new();
        for item in abi_array {
            let fn_name = item["name"].as_str().unwrap_or("");
            let fn_type = item["type"].as_str().unwrap_or("");
            let state_mutability = item["stateMutability"].as_str().unwrap_or("");
            let inputs = item["inputs"].as_array().unwrap_or(&vec![]).to_vec();
            let outputs = item["outputs"].as_array().unwrap_or(&vec![]).to_vec();

            if fn_type != "function" {
                continue;
            }

            // Generate test logic based on the function
            if fn_name == "set" && inputs.len() == 1 && inputs[0]["type"] == "uint256" {
                test_body.push_str(
                    r#"
            contract.set(U256::from(42))?;
"#,
                );
            } else if fn_name == "get"
                && outputs.len() == 1
                && outputs[0]["type"] == "uint256"
                && state_mutability == "view"
            {
                test_body.push_str(
                    r#"
            let retrieved = contract.get()?;
            println!("Retrieved value via get: {:?}", retrieved);
            assert_eq!(retrieved, U256::from(42));
"#,
                );
            }
        }

        // Only add the test if we generated some test body
        if !test_body.is_empty() {
            test_file_content.push_str(&format!(
                r#"
    #[test]
    fn test_{contract_struct_name}() -> anyhow::Result<()> {{
        #[cfg(feature = "abi-import")]
        {{
            import!("{out_dir}/Storage.sol/{file_name}");
            let contract_address = Address::from(zint::primitives::CONTRACT);
            println!("Contract address: {{contract_address:?}}");
            let contract = {contract_struct_name}::new(contract_address);
            {test_body}
            // Check storage directly
            let mut evm = contract.evm.clone();
            let storage_key = zink::storage::Mapping::<u8, U256>::storage_key(0);
            let stored_value = U256::from_be_bytes(
                evm.storage(*contract_address.as_bytes(), storage_key).unwrap_or([0u8; 32])
            );
            println!("Stored value in EVM: {{stored_value:?}}");
            assert_eq!(stored_value, U256::from(42));
            Ok(())
        }}
        #[cfg(not(feature = "abi-import"))]
        {{
            println!("Test skipped: abi-import feature not enabled");
            Ok(())
        }}
    }}
"#,
            ));
        } else {
            println!("No testable functions found in ABI for {}", file_name);
        }
    }

    test_file_content.push_str("}\n");

    // Write ztests/src/lib.rs
    fs::write(ztests_src_path.join("lib.rs"), test_file_content)
        .context("Failed to write ztests/src/lib.rs")?;

    println!("Created ztests crate at {}", ztests_path.display());
    Ok(())
}

pub fn run_ztests() -> Result<()> {
    // Find ztests directory
    let ztests_path = find_up("ztests/Cargo.toml")?
        .parent()
        .unwrap()
        .to_path_buf();

    // Deploy contracts before running tests
    let outputs = Contract::find_foundry_outputs()?;
    if outputs.is_empty() {
        println!("No Foundry outputs found");
        return Err(anyhow::anyhow!("No contracts to deploy"));
    }
    for (contract_name, _abi_path, bytecode) in outputs {
        println!(
            "Deploying contract {} with bytecode size {}",
            contract_name,
            bytecode.len()
        );
        let mut contract = Contract {
            wasm: bytecode,
            ..Default::default()
        };
        let evm = contract.deploy()?;
        evm.commit(true);
        println!(
            "Deployed contract {} at address {:?}",
            contract_name, contract.address
        );
    }

    let status = std::process::Command::new("cargo")
        .args(["nextest", "run", "--manifest-path", "ztests/Cargo.toml"])
        .current_dir(ztests_path.parent().unwrap())
        .status()
        .context("Failed to run cargo nextest")?;

    if !status.success() {
        anyhow::bail!("Tests failed");
    }

    Ok(())
}