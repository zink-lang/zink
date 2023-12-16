//! Solidity ABI abstraction.

use crate::Arg;
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
    /// An array of arguments.
    pub inputs: Vec<Arg>,
    /// An array of arguments, similar to inputs.
    pub outputs: Vec<Arg>,
}

#[cfg(feature = "syn")]
impl From<&syn::Signature> for Abi {
    fn from(sig: &syn::Signature) -> Self {
        let inputs = sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(syn::PatType { ty, .. }) = arg {
                    Some(Arg {
                        name: sig.ident.to_string(),
                        ty: crate::Param::from(ty),
                    })
                } else {
                    None
                }
            })
            .collect();

        let outputs = if let syn::ReturnType::Type(_, ty) = &sig.output {
            vec![Arg {
                name: sig.ident.to_string(),
                ty: crate::Param::from(ty),
            }]
        } else {
            vec![]
        };

        let name = sig.ident.to_string();
        Abi {
            ty: Type::from(name.as_str()),
            name,
            inputs,
            outputs,
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
