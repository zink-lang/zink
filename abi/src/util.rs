//! Zink ABI utils
use sha3::{Digest, Keccak256};

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
