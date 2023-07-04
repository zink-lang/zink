pub use self::{
    result::{Error, Result},
    wasm::WasmBuilder,
};

mod result;
mod wasm;

/// Compliation profile.
#[derive(PartialEq, Eq)]
pub enum Profile {
    Debug,
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
