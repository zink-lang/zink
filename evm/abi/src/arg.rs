//! Arg of solidity ABI.

use core::{convert::Infallible, fmt, str::FromStr};

#[cfg(not(feature = "std"))]
use crate::std::{String, ToString};

/// Arg of solidity ABI.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arg {
    /// Name of the input.
    pub name: String,
    /// Type of the input.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: Param,
}

/// The canonical type of the parameter.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Param {
    /// A 8-bit integer.
    Int8,
    /// A 16-bit integer.
    Int16,
    /// A 32-bit integer.
    Int32,
    /// A 64-bit integer.
    Int64,
    /// A 8-bit unsigned integer.
    UInt8,
    /// A 16-bit unsigned integer.
    UInt16,
    /// A 32-bit unsigned integer.
    UInt32,
    /// A 64-bit unsigned integer.
    UInt64,
    /// A boolean type.
    Bool,
    /// An EVM address.
    Address,
    /// A byte array.
    #[default]
    Bytes,
    /// A string type.
    String,
    /// An unknown type.
    Unknown(String),
}

impl From<&str> for Param {
    fn from(s: &str) -> Self {
        match s {
            "i8" | "int8" => Param::Int8,
            "u8" | "uint8" => Param::UInt8,
            "i32" | "int32" => Param::Int32,
            "i64" | "int64" => Param::Int64,
            "u16" | "uint16" => Param::UInt16,
            "u32" | "uint32" => Param::UInt32,
            "u64" | "uint64" => Param::UInt64,
            "bool" | "boolean" => Param::Bool,
            "address" | "Address" => Param::Address,
            "Bytes" | "Vec<u8>" => Param::Bytes,
            "String" => Param::String,
            _ => Param::Unknown(s.to_string()),
        }
    }
}

impl FromStr for Param {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl AsRef<str> for Param {
    fn as_ref(&self) -> &str {
        match self {
            Param::Int8 => "int8",
            Param::Int16 => "int16",
            Param::Int32 => "int32",
            Param::Int64 => "int64",
            Param::UInt8 => "uint8",
            Param::UInt16 => "uint16",
            Param::UInt32 => "uint32",
            Param::UInt64 => "uint64",
            Param::Address => "address",
            Param::Bool => "boolean",
            Param::Bytes => "bytes",
            Param::String => "string",
            Param::Unknown(ty) => ty.as_ref(),
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p: &str = self.as_ref();
        write!(f, "{p}")
    }
}

#[cfg(feature = "syn")]
impl From<&Box<syn::Type>> for Param {
    fn from(ty: &Box<syn::Type>) -> Self {
        use quote::ToTokens;

        let ident = ty.into_token_stream().to_string();
        Self::from(ident.as_str())
    }
}
