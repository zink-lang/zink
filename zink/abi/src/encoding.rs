//! ABI encoding and decoding functionality for Ethereum ABI.

use crate::result::{Result, Error};

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
    /// Invalid input data
    InvalidData,
    /// Unsupported type
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

// For String type
impl AbiEncode for String {
    fn abi_encode(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let length = bytes.len();
        
        // Dynamic data is represented as:
        // 1. An offset (32 bytes) pointing to the data location
        // 2. The length of the data (32 bytes)
        // 3. The actual data, padded to a multiple of 32 bytes
        
        // For now, I'm assuming this is the only/first dynamic parameter
        // so the offset is always 32 (pointing right after the offset itself)
        let mut result = vec![0u8; 32];
        result[31] = 32; // Offset to data location (32 bytes)
        
        // encode the length
        let mut length_bytes = vec![0u8; 32];
        length_bytes[28..32].copy_from_slice(&(length as u32).to_be_bytes());
        result.extend_from_slice(&length_bytes);
        
        // Encode the data with padding
        result.extend_from_slice(bytes);
        
        // Pad to a multiple of 32 bytes
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
        
        // Read the offset to the data
        let mut offset_bytes = [0u8; 4];
        offset_bytes.copy_from_slice(&data[28..32]);
        let offset = u32::from_be_bytes(offset_bytes) as usize;
        
        if data.len() < offset + 32 {
            return Err(DecodeError::InvalidData);
        }
        
        // Read the length of the string
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&data[offset + 28..offset + 32]);
        let length = u32::from_be_bytes(length_bytes) as usize;
        
        if data.len() < offset + 32 + length {
            return Err(DecodeError::InvalidData);
        }
        
        // Extract the string data
        let string_data = &data[offset + 32..offset + 32 + length];
        String::from_utf8(string_data.to_vec())
            .map_err(|_| DecodeError::InvalidData)
    }
}

impl<T: AbiEncode> AbiEncode for Vec<T> {
    fn abi_encode(&self) -> Vec<u8> {
        let length = self.len();
        
        // Dynamic arrays follow the same pattern as strings:
        // 1. An offset (32 bytes) pointing to the data location
        // 2. The length of the array (32 bytes)
        // 3. The elements of the array, each encoded according to its type
        
        // For now, we're assuming this is the only/first dynamic parameter
        let mut result = vec![0u8; 32];
        result[31] = 32; // Offset to data location (32 bytes)
        
        // Encode the length
        let mut length_bytes = vec![0u8; 32];
        length_bytes[28..32].copy_from_slice(&(length as u32).to_be_bytes());
        result.extend_from_slice(&length_bytes);
        
        // Encode each element
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
        
        // Read the offset to the data
        let mut offset_bytes = [0u8; 4];
        offset_bytes.copy_from_slice(&data[28..32]);
        let offset = u32::from_be_bytes(offset_bytes) as usize;
        
        if data.len() < offset + 32 {
            return Err(DecodeError::InvalidData);
        }
        
        // Read the length of the array
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

// For npw, I can't directly import from zink, 
// that's why i'm using feature flags for Address/U256
#[cfg(feature = "primitives")]
mod primitives_impl {
    use super::{AbiEncode, AbiDecode, DecodeError};
    
    // TODO: implement primitives when feature is enabled
}

/// Encode a value according to Ethereum ABI rules
pub fn encode<T: AbiEncode>(value: &T) -> Vec<u8> {
    value.abi_encode()
}

/// Decode a value from Ethereum ABI encoded bytes
pub fn decode<T: AbiDecode>(data: &[u8]) -> Result<T> {
    T::abi_decode(data).map_err(Into::into)
}

// Checks if a type is a dynamic type
pub fn is_dynamic_type(solidity_type: &str) -> bool {
    solidity_type == "string" || 
    solidity_type == "bytes" || 
    solidity_type.ends_with("[]")
}