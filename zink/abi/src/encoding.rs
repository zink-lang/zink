//! ABI encoding and decoding functionality for Ethereum ABI.

use crate::result::{Error, Result};

/// ABI encode a value
pub trait AbiEncode {
    /// Encode the value according to Ethereum ABI rules
    fn abi_encode(&self) -> Vec<u8>;
}

/// ABI decode a value
pub trait AbiDecode: Sized {
    /// Decode a value from Ethereum ABI encoded bytes
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError>;
}

/// Errors that can occur during ABI decoding
#[derive(Debug)]
pub enum DecodeError {
    InvalidData,
    UnsupportedType,
}

impl From<DecodeError> for Error {
    fn from(_e: DecodeError) -> Self {
        Error::Postcard(postcard::Error::SerializeBufferFull)
    }
}

impl AbiEncode for bool {
    fn abi_encode(&self) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        if *self {
            result[31] = 1;
        }
        result
    }
}

impl AbiDecode for bool {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 32 {
            return Err(DecodeError::InvalidData);
        }
        Ok(data[31] != 0)
    }
}

impl AbiEncode for u8 {
    fn abi_encode(&self) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        result[31] = *self;
        result
    }
}

impl AbiDecode for u8 {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 32 {
            return Err(DecodeError::InvalidData);
        }
        Ok(data[31])
    }
}

impl AbiEncode for u16 {
    fn abi_encode(&self) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        result[30..32].copy_from_slice(&self.to_be_bytes());
        result
    }
}

impl AbiDecode for u16 {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 32 {
            return Err(DecodeError::InvalidData);
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&data[30..32]);
        Ok(u16::from_be_bytes(bytes))
    }
}

impl AbiEncode for u32 {
    fn abi_encode(&self) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        result[28..32].copy_from_slice(&self.to_be_bytes());
        result
    }
}

impl AbiDecode for u32 {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 32 {
            return Err(DecodeError::InvalidData);
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data[28..32]);
        Ok(u32::from_be_bytes(bytes))
    }
}

impl AbiEncode for u64 {
    fn abi_encode(&self) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        result[24..32].copy_from_slice(&self.to_be_bytes());
        result
    }
}

impl AbiDecode for u64 {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 32 {
            return Err(DecodeError::InvalidData);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[24..32]);
        Ok(u64::from_be_bytes(bytes))
    }
}

impl AbiEncode for u128 {
    fn abi_encode(&self) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        result[16..32].copy_from_slice(&self.to_be_bytes());
        result
    }
}

impl AbiDecode for u128 {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 32 {
            return Err(DecodeError::InvalidData);
        }
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&data[16..32]);
        Ok(u128::from_be_bytes(bytes))
    }
}

// ### Standalone Functions for Address and U256

/// Encode an address (20 bytes) into ABI format
#[allow(dead_code)]
pub fn encode_address(addr: &[u8; 20]) -> Vec<u8> {
    let mut result = vec![0u8; 32];
    result[12..32].copy_from_slice(addr);
    result
}

/// Decode an address from ABI-encoded data
#[allow(dead_code)]
pub fn decode_address(data: &[u8]) -> std::result::Result<[u8; 20], DecodeError> {
    if data.len() < 32 {
        return Err(DecodeError::InvalidData);
    }
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&data[12..32]);
    Ok(bytes)
}

/// Encode a U256 (32 bytes) into ABI format
#[allow(dead_code)]
pub fn encode_u256(value: &[u8; 32]) -> Vec<u8> {
    value.to_vec()
}

/// Decode a U256 from ABI-encoded data
#[allow(dead_code)]
pub fn decode_u256(data: &[u8]) -> std::result::Result<[u8; 32], DecodeError> {
    if data.len() < 32 {
        return Err(DecodeError::InvalidData);
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&data[0..32]);
    Ok(bytes)
}

impl AbiEncode for String {
    fn abi_encode(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let length = bytes.len();

        let mut result = vec![0u8; 32];
        result[28..32].copy_from_slice(&(32u32).to_be_bytes()); // Offset is 32 bytes

        let mut length_bytes = vec![0u8; 32];
        length_bytes[28..32].copy_from_slice(&(length as u32).to_be_bytes());
        result.extend_from_slice(&length_bytes);

        result.extend_from_slice(bytes);
        let padding_needed = (32 - (bytes.len() % 32)) % 32;
        result.extend(vec![0u8; padding_needed]);

        result
    }
}

impl AbiDecode for String {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 64 {
            return Err(DecodeError::InvalidData);
        }

        let mut offset_bytes = [0u8; 4];
        offset_bytes.copy_from_slice(&data[28..32]);
        let offset = u32::from_be_bytes(offset_bytes) as usize;

        if data.len() < offset + 32 {
            return Err(DecodeError::InvalidData);
        }

        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&data[offset + 28..offset + 32]);
        let length = u32::from_be_bytes(length_bytes) as usize;

        if data.len() < offset + 32 + length {
            return Err(DecodeError::InvalidData);
        }

        let string_data = &data[offset + 32..offset + 32 + length];
        String::from_utf8(string_data.to_vec()).map_err(|_| DecodeError::InvalidData)
    }
}

impl<T: AbiEncode> AbiEncode for Vec<T> {
    fn abi_encode(&self) -> Vec<u8> {
        let length = self.len();

        let mut result = vec![0u8; 32];
        result[28..32].copy_from_slice(&(32u32).to_be_bytes()); // Offset is 32 bytes

        let mut length_bytes = vec![0u8; 32];
        length_bytes[28..32].copy_from_slice(&(length as u32).to_be_bytes());
        result.extend_from_slice(&length_bytes);

        for element in self {
            result.extend_from_slice(&element.abi_encode());
        }

        result
    }
}

impl<T: AbiDecode> AbiDecode for Vec<T> {
    fn abi_decode(data: &[u8]) -> std::result::Result<Self, DecodeError> {
        if data.len() < 64 {
            return Err(DecodeError::InvalidData);
        }

        let mut offset_bytes = [0u8; 4];
        offset_bytes.copy_from_slice(&data[28..32]);
        let offset = u32::from_be_bytes(offset_bytes) as usize;

        if data.len() < offset + 32 {
            return Err(DecodeError::InvalidData);
        }

        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&data[offset + 28..offset + 32]);
        let length = u32::from_be_bytes(length_bytes) as usize;

        let mut result = Vec::with_capacity(length);
        let mut pos = offset + 32;

        for _ in 0..length {
            if data.len() < pos + 32 {
                return Err(DecodeError::InvalidData);
            }

            let element = T::abi_decode(&data[pos..pos + 32])?;
            result.push(element);
            pos += 32;
        }

        Ok(result)
    }
}

pub fn encode<T: AbiEncode>(value: &T) -> Vec<u8> {
    value.abi_encode()
}

pub fn decode<T: AbiDecode>(data: &[u8]) -> Result<T> {
    T::abi_decode(data).map_err(Into::into)
}

pub fn is_dynamic_type(solidity_type: &str) -> bool {
    solidity_type == "string" || solidity_type == "bytes" || solidity_type.ends_with("[]")
}
