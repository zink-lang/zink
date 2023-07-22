//! WASM local slot.

use crate::abi::{ToLSBytes, Type};
use smallvec::SmallVec;
use wasmparser::ValType;

/// A local slot.
///
/// Represents the type, location and addressing mode of a local
/// in the stack's local and argument area.
#[derive(Debug)]
pub struct LocalSlot {
    /// The type contained by this local slot.
    inner: ValType,
}

impl Type for LocalSlot {
    fn size(&self) -> usize {
        self.inner.size()
    }
}

impl From<ValType> for LocalSlot {
    fn from(inner: ValType) -> Self {
        Self { inner }
    }
}

impl From<i32> for LocalSlot {
    fn from(_inner: i32) -> Self {
        Self {
            inner: ValType::I32,
        }
    }
}

/// Solidity's implementation uses 16 slots for locals.
/// ref: <https://docs.soliditylang.org/en/v0.8.20/internals/optimizer.html#stackcompressor>
#[derive(Default, Debug)]
pub struct Locals {
    inner: SmallVec<[LocalSlot; 16]>,
}

impl Locals {
    /// Get local from index.
    pub fn get(&self, index: usize) -> &LocalSlot {
        &self.inner[index]
    }

    /// Get the lower significant bytes of the offset of a local.
    ///
    /// TODO: considering if it is necessary to store the offset
    /// of each slots. (guess not)
    pub fn offset_of(&self, index: usize) -> SmallVec<[u8; 32]> {
        self.inner[..index]
            .iter()
            .fold(0, |acc, x| acc + x.align())
            .to_ls_bytes()
            .to_vec()
            .into()
    }

    /// Push a local slot.
    pub fn push(&mut self, slot: impl Into<LocalSlot>) {
        self.inner.push(slot.into())
    }
}
