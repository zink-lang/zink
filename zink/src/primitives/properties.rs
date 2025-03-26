use super::{Address, Bytes32};
use crate::ffi;

/// Get the current block number.
pub fn number() -> u64 {
    unsafe { ffi::evm::number() }
}

/// Get the hash of one of the 256 most recent complete blocks.
pub fn blockhash(block_number: u64) -> Bytes32 {
    unsafe { ffi::evm::blockhash(block_number) }
}

/// Get versioned hashes.
pub fn blobhash(index: u64) -> Bytes32 {
    unsafe { ffi::evm::blobhash(index) }
}

/// Get the current block’s base fee.
pub fn basefee() -> u64 {
    unsafe { ffi::evm::basefee() }
}

/// Get the current block’s blob base fee.
pub fn blobbasefee() -> u64 {
    unsafe { ffi::evm::blobbasefee() }
}

/// Get the current chain id.
pub fn chainid() -> u64 {
    unsafe { ffi::evm::chainid() }
}

/// Get the block’s beneficiary address.
pub fn coinbase() -> Address {
    unsafe { ffi::evm::coinbase() }
}

/// Get the previous block’s RANDAO mix.
pub fn prevrandao() -> Bytes32 {
    unsafe { ffi::evm::prevrandao() }
}

/// Get the current block gaslimit.
pub fn gaslimit() -> Bytes32 {
    unsafe { ffi::evm::gaslimit() }
}

/// Get the amount of available gas.
pub fn gasleft() -> Bytes32 {
    unsafe { ffi::evm::gas() }
}

/// Get the block’s timestamp.
pub fn timestamp() -> u64 {
    unsafe { ffi::evm::timestamp() }
}

/// Get the gas price of the transaction.
pub fn gasprice() -> u64 {
    unsafe { ffi::evm::gasprice() }
}
