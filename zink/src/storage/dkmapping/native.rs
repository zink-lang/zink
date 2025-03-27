use crate::{ffi, Asm};

pub fn sstore() {
    ffi::evm::sstore()
}

pub fn tstore() {
    ffi::evm::tstore()
}

/// Load storage key to stack
#[inline(always)]
pub fn load_double_key(key1: impl Asm, key2: impl Asm, index: i32) {
    ffi::label_reserve_mem_64();

    // write key1 to memory
    key1.push();
    ffi::evm::push0();
    ffi::evm::mstore();

    // write index to memory
    index.push();
    ffi::asm::push_u8(0x20);
    ffi::evm::mstore();

    // hash key
    ffi::asm::push_u8(0x40);
    ffi::evm::push0();
    ffi::evm::keccak256();

    // stores the hash
    ffi::evm::push0();
    ffi::evm::mstore();

    // write index to memory
    key2.push();
    ffi::asm::push_u8(0x20);
    ffi::evm::mstore();

    // hash key
    ffi::asm::push_u8(0x40);
    ffi::evm::push0();
    ffi::evm::keccak256();
}
