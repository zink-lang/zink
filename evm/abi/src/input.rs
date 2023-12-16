//! Input of solidity ABI.

/// Input of solidity ABI.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Input {
    /// Name of the input.
    pub name: String,
    /// Type of the input.
    pub ty: Param,
}

/// The canonical type of the parameter.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Param {
    /// A 32-bit integer.
    Int32,
    /// A 64-bit integer.
    Int64,
    /// A 32-bit unsigned integer.
    UInt32,
    /// A 64-bit unsigned integer.
    UInt64,
}

impl AsRef<str> for Param {
    fn as_ref(&self) -> &str {
        match self {
            Param::Int32 => "int32",
            Param::Int64 => "int64",
            Param::UInt32 => "uint32",
            Param::UInt64 => "uint64",
        }
    }
}

impl ToString for Param {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}
