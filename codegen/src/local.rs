//! An abstraction to read the defined locals from the Wasm binary for a function.

use std::ops::Range;

use crate::{abi, LocalSlot, Result};
use smallvec::SmallVec;
use wasmparser::{BinaryReader, FuncValidator, ValidatorResources};

/// Solidity's implementation uses 16 slots for locals, so we do the same.
///
/// ref: https://docs.soliditylang.org/en/v0.8.20/internals/optimizer.html#stackcompressor
pub type Locals = SmallVec<[LocalSlot; 16]>;

/// Function defined locals start and end in the frame.
#[derive(Default)]
pub struct DefinedLocalsRange(Range<u32>);

/// An abstraction to read the defined locals from the Wasm binary for a function.
#[derive(Default)]
pub struct DefinedLocals {
    /// The defined locals for a function.
    pub defined_locals: Locals,
    /// The size of the defined locals.
    pub stack_size: u32,
}

impl DefinedLocals {
    /// Compute the local slots for a Wasm function.
    pub fn new(
        reader: &mut BinaryReader<'_>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<Self> {
        let mut next_stack = 0;
        // The first 32 bits of a Wasm binary function describe the number of locals.
        let local_count = reader.read_var_u32()?;
        let mut slots: Locals = Default::default();

        for _ in 0..local_count {
            let position = reader.original_position();
            let count = reader.read_var_u32()?;
            let ty = reader.read()?;
            validator.define_locals(position, count, ty)?;

            for _ in 0..count {
                let ty_size = abi::ty_size(&ty);
                next_stack = abi::align_to(next_stack, ty_size) + ty_size;
                slots.push(LocalSlot::new(ty, next_stack));
            }
        }

        Ok(Self {
            defined_locals: slots,
            stack_size: next_stack,
        })
    }
}
