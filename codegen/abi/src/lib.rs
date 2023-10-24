//! Utils for generating of zink ABI
pub use self::result::{Error, Result};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

mod result;

/// Generate a keccak hash of the input (sha3)
pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    hasher.finalize().into()
}

/// Get function selector from function signature.
pub fn selector(input: &[u8]) -> [u8; 4] {
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&keccak256(input)[..4]);

    selector
}

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
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&keccak256(sig.as_bytes())[..4]);

        selector
    }

    /// Parse ABI from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        postcard::from_bytes(bytes).map_err(Into::into)
    }

    /// Decode ABI form hex string.
    pub fn from_hex(hex: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(hex)?)
    }

    /// Decode ABI form hex string.
    pub fn from_hex_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_hex(&String::from_utf8_lossy(bytes))
    }

    /// Convert ABI to hex string.
    pub fn to_hex(&self) -> Result<String> {
        self.to_bytes().map(hex::encode)
    }

    /// Convert ABI to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        postcard::to_stdvec(&self).map_err(Into::into)
    }
}
