//! Zink ABI implementation
//!
//! Currently just a wrapper of solidity ABI.

pub mod result;
pub mod selector;

use core::ops::{Deref, DerefMut};

/// Function ABI.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abi(evm_abi::Abi);

impl Deref for Abi {
    type Target = evm_abi::Abi;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Abi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "bytes")]
impl Abi {
    /// Convert [`Abi`] to bytes.
    pub fn to_bytes(&self) -> postcard::Result<Vec<u8>> {
        postcard::to_stdvec(self).map_err(Into::into)
    }

    /// Convert bytes to [`Abi`].
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> postcard::Result<Self> {
        postcard::from_bytes(bytes.as_ref()).map_err(Into::into)
    }
}

#[cfg(feature = "hex")]
mod hex_impl {
    use crate::{result::Result, Abi};

    impl Abi {
        /// Convert [`Abi`] to hex string.
        pub fn to_hex(&self) -> Result<String> {
            Ok("0x".to_string() + &hex::encode(self.to_bytes()?))
        }

        /// Convert hex string to [`Abi`].
        pub fn from_hex(hex: impl AsRef<str>) -> Result<Self> {
            Self::from_bytes(hex::decode(hex.as_ref().trim_start_matches("0x"))?)
                .map_err(Into::into)
        }
    }

    impl ToString for Abi {
        fn to_string(&self) -> String {
            self.to_hex().unwrap_or_default()
        }
    }

    impl core::str::FromStr for Abi {
        type Err = crate::result::Error;

        fn from_str(hex: &str) -> Result<Self> {
            Self::from_hex(hex)
        }
    }
}

#[cfg(feature = "syn")]
impl From<&syn::Signature> for Abi {
    fn from(sig: &syn::Signature) -> Self {
        Self(evm_abi::Abi::from(sig))
    }
}
