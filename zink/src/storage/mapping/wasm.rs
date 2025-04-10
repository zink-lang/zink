use crate::{ffi, Asm};

pub fn sstore() {
    unsafe { ffi::evm::sstore() }
}

pub fn tstore() {
    unsafe { ffi::evm::tstore() }
}

/// Load storage key to stack
pub fn load_key(key: impl Asm, index: i32) {
    unsafe {
        ffi::label_reserve_mem_32();

        // write key to memory
        key.push();
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
    }
}
