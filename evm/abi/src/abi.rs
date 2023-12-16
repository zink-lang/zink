//! Solidity ABI abstraction.
use crate::Input;

/// Solidity ABI abstraction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abi {
    /// ABI name.
    pub name: String,
    /// ABI type.
    pub ty: Type,
    /// ABI inputs.
    pub inputs: Vec<Input>,
}

/// Solidity ABI type.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// Constructor ABI.
    Constructor,
    /// Function ABI.
    Function,
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        match self {
            Type::Constructor => "constructor",
            Type::Function => "function",
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}
