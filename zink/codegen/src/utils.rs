//! Utils for code generation of zink APIs
use sha3::{Digest, Keccak256};

/// Generate a keccak hash of the input (sha3)
pub fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    hasher.finalize().into()
}
