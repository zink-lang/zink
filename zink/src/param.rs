//! Types loader

use crate::{ffi, storage::StorageValue, Asm};

/// Param for non-primitive types
#[derive(Clone, Copy)]
pub struct ParamBytes<const S: u64> {
    ptr: u32,
}

impl<const S: u64> Asm for ParamBytes<S> {
    fn push(self) {
        unsafe { ffi::pload(self.ptr, S) }
    }
}

impl StorageValue for ParamBytes<20> {
    fn sload() -> Self {
        unsafe { ffi::asm::sload_pbytes20() }
    }
}
