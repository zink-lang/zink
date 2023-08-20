//! CLI Utils
#![cfg(feature = "elko")]

pub use self::{
    manifest::Manifest,
    result::{Error, Result},
    wasm::WasmBuilder,
};

mod manifest;
mod result;
mod wasm;

/// Compilation profile.
#[derive(PartialEq, Eq)]
pub enum Profile {
    /// Debug profile.
    Debug,
    /// Release profile.
    Release,
}

impl From<&str> for Profile {
    fn from(profile: &str) -> Self {
        match profile {
            "release" | "production" => Profile::Release,
            _ => Profile::Debug,
        }
    }
}

impl AsRef<str> for Profile {
    fn as_ref(&self) -> &str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
        }
    }
}
