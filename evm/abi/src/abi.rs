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
