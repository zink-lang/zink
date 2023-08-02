//! WASM local slot.

use crate::{
    abi::{ToLSBytes, Type},
    Error, Result,
};
use smallvec::SmallVec;
use wasmparser::ValType;

/// The type of a local slot.
#[derive(Debug, PartialEq, Eq)]
pub enum LocalSlotType {
    /// A function parameter.
    Parameter,
    /// A local variable.
    Variable,
}

/// A local slot.
///
/// Represents the type, location and addressing mode of a local
/// in the stack's local and argument area.
#[derive(Debug)]
pub struct LocalSlot {
    /// The type contained by this local slot.
    inner: ValType,
    /// The type of this local slot.
    ty: LocalSlotType,

    /// Stack pointer of the local slot.
    pub sp: Option<usize>,
}

impl LocalSlot {
    /// Create a new local slot.
    pub fn new(inner: ValType, ty: LocalSlotType, sp: Option<usize>) -> Self {
        Self { inner, ty, sp }
    }

    /// Get the type of this local slot.
    pub fn ty(&self) -> &LocalSlotType {
        &self.ty
    }

    /// Get the value type of this local slot.
    pub fn val_ty(&self) -> &ValType {
        &self.inner
    }
}

impl Type for LocalSlot {
    fn size(&self) -> usize {
        self.inner.size()
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
    pub fn get(&self, index: usize) -> Result<&LocalSlot> {
        self.inner
            .get(index)
            .ok_or_else(|| Error::InvalidLocalIndex(index))
    }

    /// Get mutate local from index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut LocalSlot> {
        self.inner
            .get_mut(index)
            .ok_or_else(|| Error::InvalidLocalIndex(index))
    }

    /// Get the lower significant bytes of the byte offset of a local.
    ///
    /// - **Parameter**: If the local is a parameter, the offset is relative to the offset
    /// of the calldata.
    /// - **Variable**: If the local is a variable, the offset is relative to the offset
    /// of the memory.
    pub fn offset_of(&self, index: usize) -> Result<SmallVec<[u8; 32]>> {
        let local = self.get(index)?;
        let offset = if local.ty() == &LocalSlotType::Parameter {
            self.inner[..index].iter().fold(0, |acc, x| acc + x.align())
        } else {
            self.inner[..index]
                .iter()
                .filter(|x| x.ty() == &LocalSlotType::Variable)
                .fold(0, |acc, x| acc + x.align())
        }
        .to_ls_bytes()
        .to_vec()
        .into();

        Ok(offset)
    }

    /// Push a local slot.
    pub fn push(&mut self, slot: impl Into<LocalSlot>) {
        self.inner.push(slot.into())
    }

    /// Shift local stack pointers.
    pub fn shift_sp(&mut self, index: usize, sp: usize) -> Result<()> {
        let local = self.get_mut(index)?;
        let src_sp = local.sp.ok_or_else(|| Error::InvalidLocalIndex(index))?;
        local.sp = Some(sp);

        for (idx, local) in self.inner.iter_mut().enumerate() {
            if idx == index {
                continue;
            }

            if let Some(local_sp) = local.sp {
                if local_sp > src_sp {
                    // TODO: Artihmetic checks
                    local.sp = Some(local_sp - 1);
                }
            }
        }

        Ok(())
    }

    /// Get the length of locals
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
