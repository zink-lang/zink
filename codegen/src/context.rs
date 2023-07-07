//! CodeGen context

use wasmparser::Frame;

use crate::Stack;

/// The code generation context.
/// The code generation context is made up of three
/// essential data structures:
///
/// * The value stack, which keeps track of the state of the values
///   after each operation.
/// * The current function's frame.
///
/// These data structures normally require cooperating with each other
/// to perform most of the operations needed during the code
/// generation process. The code generation context should
/// be generally used as the single entry point to access
/// the compound functionality provided by its elements.
pub struct Context {
    /// The current function's frame.
    pub frame: Frame,
    /// Reachability state.
    pub reachable: bool,
    /// The value stack.
    pub stack: Stack,
}
