//! Function frame.
use crate::local::{DefinedLocals, DefinedLocalsRange, Locals};

/// Frame handler abstraction.
pub struct Frame {
    /// The size of the entire local area; the arguments plus the function defined locals.
    pub locals_size: u32,

    /// The range in the frame corresponding to the defined locals range.
    pub defined_locals_range: DefinedLocalsRange,

    /// The local slots for the current function.
    ///
    /// Locals get calculated when allocating a frame and are readonly
    /// through the function compilation lifetime.
    pub locals: Locals,
}

impl Frame {
    // /// Allocate a new frame.
    // pub fn new() -> Self {
    //     Self {
    //         locals_size,
    //         defined_locals_range,
    //         locals: Locals::new(locals_size),
    //     }
    // }
}
