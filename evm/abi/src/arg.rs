//! Arg of solidity ABI.

use core::{convert::Infallible, str::FromStr};

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
    /// A 32-bit integer.
    Int32,
    /// A 64-bit integer.
    Int64,
    /// A 32-bit unsigned integer.
    UInt32,
    /// A 64-bit unsigned integer.
    UInt64,
    /// An EVM address.
    Address,
    /// A boolean type.
    Bool,
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
            "i32" | "int32" => Param::Int32,
            "i64" | "int64" => Param::Int64,
            "u32" | "uint32" => Param::UInt32,
            "usize" | "u64" | "uint64" => Param::UInt64,
            "Address" => Param::Address,
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
            Param::Int32 => "int32",
            Param::Int64 => "int64",
            Param::UInt32 => "uint32",
            Param::UInt64 => "uint64",
            Param::Address => "address",
            Param::Bool => "bool",
            Param::Bytes => "bytes",
            Param::String => "string",
            Param::Unknown(ty) => ty.as_ref(),
        }
    }
}

impl ToString for Param {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
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
