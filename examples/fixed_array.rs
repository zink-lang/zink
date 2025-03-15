//! Fixed array example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::revert;

// Define a minimal Address type for the contract
#[derive(Copy, Clone, PartialEq)]
pub struct Address([u8; 20]);

/// Test function with fixed array parameter
#[zink::external]
pub fn method_with_fix_array_as_parameter(offset: u32) {
    let addr0 = Address([0u8; 20]);
    let mut loaded_addr = Address([0u8; 20]);

    // Load the first 20 bytes from memory at the offset
    for i in 0..20 {
        unsafe {
            loaded_addr.0[i] = *(offset as *const u8).add(i);
        }
    }

    if loaded_addr != addr0 {
        revert!("Address mismatch");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use zingen::masm::MacroAssembler;
    use zingen::wasm::{
        abi::{Address as CodegenAddress, FixedArray},
        ToLSBytes,
    };
    use zint::{Bytes32, Contract};

    // Helper to convert our Address to test format
    impl Address {
        fn to_test_bytes(&self) -> Vec<u8> {
            self.0.to_vec()
        }
    }

    #[test]
    fn test_address_to_ls_bytes() {
        let addr = CodegenAddress([1u8; 20]);
        assert_eq!(addr.to_ls_bytes().to_vec(), vec![1u8; 20]);
    }

    #[test]
    fn test_fixed_array_to_ls_bytes() {
        let array = FixedArray::new(vec![
            CodegenAddress([1u8; 20]),
            CodegenAddress([2u8; 20]),
            CodegenAddress([3u8; 20]),
        ]);
        let bytes = array.to_ls_bytes();
        assert_eq!(bytes.len(), 60); // 3 * 20
        assert_eq!(&bytes[0..20], &[1u8; 20]);
        assert_eq!(&bytes[20..40], &[2u8; 20]);
        assert_eq!(&bytes[40..60], &[3u8; 20]);
    }

    #[test]
    fn test_fixed_array_parameter() -> anyhow::Result<()> {
        let mut contract = Contract::search("fixed_array")?.compile()?;

        let addr0 = Address([0u8; 20]);
        let addr1 = {
            let mut arr = [0u8; 20];
            arr[16..20].copy_from_slice(&1u32.to_le_bytes());
            Address(arr)
        };
        let addr2 = {
            let mut arr = [0u8; 20];
            arr[16..20].copy_from_slice(&2u32.to_le_bytes());
            Address(arr)
        };
        let wrong_addr0 = {
            let mut arr = [0u8; 20];
            arr[16..20].copy_from_slice(&999u32.to_le_bytes());
            Address(arr)
        };

        let mut asm = MacroAssembler::default();
        let array_data = [
            addr0.to_test_bytes(),
            addr1.to_test_bytes(),
            addr2.to_test_bytes(),
        ]
        .concat();
        let memory_info = asm.memory_write_bytes(&array_data)?;
        let offset = u32::from_le_bytes(memory_info.offset.as_ref().try_into().unwrap());

        asm.push(&offset.to_le_bytes())?;
        let info = contract.execute(&[
            b"method_with_fix_array_as_parameter(uint32)".to_vec(),
            offset.to_le_bytes().to_vec(),
        ])?;
        assert!(info.ret.is_empty(), "Expected no return value on success");

        let wrong_array_data = [
            wrong_addr0.to_test_bytes(),
            addr1.to_test_bytes(),
            addr2.to_test_bytes(),
        ]
        .concat();
        let wrong_memory_info = asm.memory_write_bytes(&wrong_array_data)?;
        let wrong_offset =
            u32::from_le_bytes(wrong_memory_info.offset.as_ref().try_into().unwrap());
        let info = contract.execute(&[
            b"method_with_fix_array_as_parameter(uint32)".to_vec(),
            wrong_offset.to_le_bytes().to_vec(),
        ]);
        assert!(info.is_err(), "Expected revert with incorrect address");

        Ok(())
    }
}
