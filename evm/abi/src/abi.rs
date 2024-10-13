//! Solidity ABI abstraction.

use crate::Arg;
use core::{convert::Infallible, fmt, str::FromStr};

#[cfg(feature = "syn")]
use quote::ToTokens;

#[cfg(not(feature = "std"))]
use crate::std::{String, ToString, Vec};

/// Solidity ABI abstraction.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abi {
    /// ABI name.
    pub name: String,
    /// ABI type.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
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
                if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = arg {
                    Some(Arg {
                        name: pat.to_token_stream().to_string(),
                        ty: crate::Param::from(ty),
                    })
                } else {
                    None
                }
            })
            .collect();

        let outputs = if let syn::ReturnType::Type(_, ty) = &sig.output {
            vec![Arg {
                // TODO: how to name the output?
                name: "output".into(),
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
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty: &str = self.as_ref();
        write!(f, "{ty}")
    }
}
