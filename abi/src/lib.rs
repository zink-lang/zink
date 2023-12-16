//! EVM ABI implementation
//!
//! https://docs.soliditylang.org/en/latest/abi-spec.html#json
pub use self::result::{Error, Result};
use serde::{Deserialize, Serialize};

mod codec;
mod result;
pub mod util;

/// Function ABI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abi {
    /// Function name.
    pub name: String,
    /// Function inputs.
    pub inputs: Vec<String>,
}

impl Abi {
    /// Get function signature.
    pub fn signature(&self) -> String {
        self.name.clone() + "(" + &self.inputs.join(",") + ")"
    }

    /// Get function selector.
    pub fn selector(&self) -> [u8; 4] {
        let sig = self.signature();
        util::selector(sig.as_bytes())
    }
}
