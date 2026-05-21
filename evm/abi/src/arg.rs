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
    /// A 256-bit unsigned integer.
    UInt256,
    // /// A 256-bit unsigned integer.
    // UInt256,
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
            "U256" | "u256" | "uint256" => Param::UInt256,
            "bool" => Param::Bool,
            "address" | "Address" => Param::Address,
            "String" | "String32" => Param::String,
            "Vec<u8>" => Param::Bytes,
            bytes if bytes.starts_with("Bytes") => Param::Bytes,
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
            Param::UInt256 => "uint256",
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

impl Param {
    /// Number of 32-byte ABI calldata slots used by this parameter.
    pub fn calldata_slots(&self) -> usize {
        self.tuple_fields().map_or(1, |fields| fields.len())
    }

    /// Byte offsets of tuple fields paired with their ABI calldata slot index.
    pub fn tuple_field_offsets(&self) -> Option<Vec<(usize, usize)>> {
        let fields = self.tuple_fields()?;
        let mut offset = 0;

        Some(
            fields
                .iter()
                .enumerate()
                .map(|(slot, field)| {
                    let size = field.abi_byte_size();
                    offset = align_to(offset, size);
                    let field_offset = offset;
                    offset += size;
                    (field_offset, slot)
                })
                .collect(),
        )
    }

    fn signature_type(&self) -> String {
        match self {
            Param::Unknown(ty) => ty.clone(),
            _ => self.as_ref().to_string(),
        }
    }

    fn tuple_fields(&self) -> Option<Vec<Param>> {
        let Param::Unknown(ty) = self else {
            return None;
        };

        let ty = ty.trim();
        if !(ty.starts_with('(') && ty.ends_with(')')) {
            return None;
        }

        let fields = split_tuple_fields(&ty[1..ty.len() - 1]);
        if fields.is_empty() {
            return None;
        }

        Some(
            fields
                .into_iter()
                .map(|field| Param::from(field.as_str()))
                .collect(),
        )
    }

    fn abi_byte_size(&self) -> usize {
        match self {
            Param::Int8 | Param::UInt8 | Param::Bool => 1,
            Param::Int16 | Param::UInt16 => 2,
            Param::Int32 | Param::UInt32 => 4,
            Param::Int64 | Param::UInt64 => 8,
            Param::Address => 20,
            Param::UInt256 | Param::Bytes | Param::String | Param::Unknown(_) => 32,
        }
    }
}

fn align_to(offset: usize, alignment: usize) -> usize {
    if alignment == 0 {
        return offset;
    }

    (offset + alignment - 1) & !(alignment - 1)
}

fn split_tuple_fields(fields: &str) -> Vec<String> {
    let mut depth = 0;
    let mut start = 0;
    let mut output = Vec::new();

    for (index, ch) in fields.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                let field = fields[start..index].trim();
                if !field.is_empty() {
                    output.push(field.to_string());
                }
                start = index + 1;
            }
            _ => {}
        }
    }

    let field = fields[start..].trim();
    if !field.is_empty() {
        output.push(field.to_string());
    }

    output
}

#[cfg(feature = "syn")]
impl From<&Box<syn::Type>> for Param {
    fn from(ty: &Box<syn::Type>) -> Self {
        Self::from(ty.as_ref())
    }
}

#[cfg(feature = "syn")]
impl From<&syn::Type> for Param {
    fn from(ty: &syn::Type) -> Self {
        use quote::ToTokens;

        match ty {
            syn::Type::Tuple(tuple) => {
                let fields = tuple
                    .elems
                    .iter()
                    .map(|field| Param::from(field).signature_type())
                    .collect::<Vec<_>>()
                    .join(",");

                Param::Unknown(format!("({fields})"))
            }
            _ => {
                let ident = ty.into_token_stream().to_string().replace(' ', "");
                Self::from(ident.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Param;

    #[test]
    fn tuple_param_uses_canonical_signature_types() {
        let ty: syn::Type = syn::parse_str("(i32, u64)").unwrap();
        let param = Param::from(&ty);

        assert_eq!(param.as_ref(), "(int32,uint64)");
        assert_eq!(param.calldata_slots(), 2);
        assert_eq!(param.tuple_field_offsets(), Some(vec![(0, 0), (8, 1)]));
    }
}
