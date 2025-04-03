use cargo_metadata::MetadataCommand;

fn main() {
    // Rerun the build script if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");

    let metadata = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .exec()
        .expect("Failed to fetch cargo metadata");

    let zink_version = metadata
        .packages
        .iter()
        .find(|p| p.name == "zinkc")
        .map(|p| p.version.to_string())
        .expect("Could not find zink dependency in metadata");

    // Emit the version as a cargo key-value pair
    println!("cargo:ZINK_VERSION={}", zink_version);
}
