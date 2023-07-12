//! Function environment

use wasmparser::FuncType;

/// Function environment
#[derive(Debug, Clone)]
pub struct FuncEnv {
    /// Index of this function.
    pub index: usize,
    /// The function type.
    pub ty: FuncType,
}
   A       ��C�U