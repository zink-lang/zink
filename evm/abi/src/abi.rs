//! Solidity ABI abstraction.

use crate::Input;
use core::{convert::Infallible, str::FromStr};

/// Solidity ABI abstraction.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abi {
    /// ABI name.
    pub name: String,
    /// ABI type.
    #[serde(rename = "type")]
    pub ty: Type,
    /// ABI inputs.
    pub inputs: Vec<Input>,
}

#[cfg(feature = "syn")]
impl From<&syn::Signature> for Abi {
    fn from(sig: &syn::Signature) -> Self {
        let args = sig.inputs.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(syn::PatType { ty, .. }) = arg {
                Some(Input {
                    name: sig.ident.to_string(),
                    ty: crate::Param::from(ty),
                })
            } else {
                None
            }
        });

        Abi {
            name: sig.ident.to_string(),
            inputs: args.collect(),
            ty: Type::Function,
        }
    }
}

/// Solidity ABI type.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(rename_all = "lowercase")]
pub enum Type {
    /// Constructor ABI.
    Constructor,
    /// Function ABI.
    #[default]
    Function,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "constructor" => Type::Constructor,
            _ => Type::Function,
        }
    }
}

impl FromStr for Type {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
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
