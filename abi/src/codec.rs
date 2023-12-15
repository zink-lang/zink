//! Serialization and Deserialization for Zink ABI.

use crate::{Abi, Error, Result};

/// Implement encoding and decoding for Zink ABI.
macro_rules! impl_codec {
    ($ty:ident) => {
        impl $ty {
            /// Convert self to bytes.
            pub fn to_bytes(&self) -> Result<Vec<u8>> {
                postcard::to_stdvec(self).map_err(Into::into)
            }

            /// Convert self to hex string.
            pub fn to_hex(&self) -> Result<String> {
                Ok(hex::encode(self.to_bytes()?))
            }

            /// Convert bytes to self.
            pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
                postcard::from_bytes(bytes.as_ref()).map_err(Into::into)
            }

            /// Convert hex string to self.
            pub fn from_hex(hex: impl AsRef<str>) -> Result<Self> {
                Self::from_bytes(&hex::decode(hex.as_ref().trim_start_matches("0x"))?)
            }
        }

        impl ToString for $ty {
            fn to_string(&self) -> String {
                self.to_hex().unwrap_or_default()
            }
        }

        impl core::str::FromStr for $ty {
            type Err = Error;

            fn from_str(hex: &str) -> Result<Self> {
                Self::from_hex(hex)
            }
        }
    };
}

impl_codec!(Abi);
