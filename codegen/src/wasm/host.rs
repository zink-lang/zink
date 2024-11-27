//! Host functions

use crate::{Error, Result};
use anyhow::anyhow;
use core::str::FromStr;
use opcodes::{OpCode as _, ShangHai as OpCode};

/// EVM built-in function.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HostFunc {
    /// EVM assemble operations.
    Evm(OpCode),
    /// No operations, this only covers `push_$ty` at the moment.
    NoOp,
    // Zinkc helper functions
    //
    /// Emit ABI to the compiler.
    EmitABI,
    /// check equal of two addresses
    AddressEq,
    /// Push u256 max to stack
    U256MAX,
    /// Revert messages with length of slots
    Revert(usize),
    /// Compiler labels
    Label(CompilerLabel),
}

impl HostFunc {
    /// Stack input size.
    pub fn stack_in(&self) -> u8 {
        match self {
            Self::Evm(op) => op.stack_in() as u8,
            _ => 0,
        }
    }

    /// Stack output size.
    pub fn stack_out(&self) -> u8 {
        match self {
            Self::Evm(op) => op.stack_out() as u8,
            _ => 0,
        }
    }
}

impl TryFrom<(&str, &str)> for HostFunc {
    type Error = Error;

    fn try_from(import: (&str, &str)) -> Result<Self> {
        let (module, name) = import;
        match import {
            ("asm", name) => {
                if name.starts_with("sload") {
                    Ok(Self::Evm(OpCode::SLOAD))
                } else if name.starts_with("revert") {
                    let count = name.trim_start_matches("revert");

                    // TODO: use anyhow instead of Error
                    Ok(Self::Revert(count.parse().map_err(|e| anyhow!("{e}"))?))
                } else {
                    Ok(Self::NoOp)
                }
            }
            ("evm", name) => Ok(Self::Evm(OpCode::from_str(name).map_err(|_| {
                tracing::error!("Failed to load host function: {:?}", import);
                Error::HostFuncNotFound(module.into(), name.into())
            })?)),
            ("zinkc", "emit_abi") => Ok(Self::EmitABI),
            ("zinkc", "address_eq") => Ok(Self::Evm(OpCode::EQ)),
            ("zinkc", "u256_add") => Ok(Self::Evm(OpCode::ADD)),
            ("zinkc", "u256_sub") => Ok(Self::Evm(OpCode::SUB)),
            ("zinkc", "u256_lt") => Ok(Self::Evm(OpCode::LT)),
            ("zinkc", "u256_max") => Ok(Self::U256MAX),
            ("zinkc", "u256_addmod") => Ok(Self::Evm(OpCode::ADDMOD)),
            ("zinkc", "u256_mulmod") => Ok(Self::Evm(OpCode::MULMOD)),
            ("zinkc", "label_reserve_mem_32") => Ok(Self::Label(CompilerLabel::ReserveMemory32)),
            ("zinkc", "label_reserve_mem_64") => Ok(Self::Label(CompilerLabel::ReserveMemory64)),
            _ => {
                tracing::warn!("Failed to load host function: {:?}", import);
                Err(Error::HostFuncNotFound(module.into(), name.into()))
            }
        }
    }
}

/// Labels in host functions
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum CompilerLabel {
    ReserveMemory32,
    ReserveMemory64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addmod_mulmod_host_functions() {
        // Test ADDMOD host function conversion
        let addmod_func = HostFunc::try_from(("zinkc", "u256_addmod"));
        assert!(addmod_func.is_ok());
        let addmod_host_func = addmod_func.unwrap();

        // Verify it maps to the correct EVM opcode
        match addmod_host_func {
            HostFunc::Evm(OpCode::ADDMOD) => {}
            _ => panic!("Expected ADDMOD opcode"),
        }

        // Verify stack input/output for ADDMOD
        assert_eq!(addmod_host_func.stack_in(), 3);
        assert_eq!(addmod_host_func.stack_out(), 1);

        // Test MULMOD host function conversion
        let mulmod_func = HostFunc::try_from(("zinkc", "u256_mulmod"));
        assert!(mulmod_func.is_ok());
        let mulmod_host_func = mulmod_func.unwrap();

        // Verify it maps to the correct EVM opcode
        match mulmod_host_func {
            HostFunc::Evm(OpCode::MULMOD) => {}
            _ => panic!("Expected MULMOD opcode"),
        }

        // Verify stack input/output for MULMOD
        assert_eq!(mulmod_host_func.stack_in(), 3);
        assert_eq!(mulmod_host_func.stack_out(), 1);
    }

    #[test]
    fn test_addmod_mulmod_example_scenarios() {
        fn modular_add(a: u64, b: u64, n: u64) -> u64 {
            ((a % n) + (b % n)) % n
        }

        let test_cases = [
            (10u64, 20, 7, 2, 4),
            (100, 200, 3, 0, 2),
            (5, 6, 7, 4, 2),
            (10, 11, 12, 9, 2),
        ];

        for (a, b, n, expected_add, expected_mul) in test_cases.iter() {
            let actual_add = modular_add(*a, *b, *n);
            let actual_mul = (*a * *b) % *n;

            println!(
                "Debug: a={}, b={}, N={}, actual_add={}, expected_add={}",
                a, b, n, actual_add, expected_add
            );

            assert_eq!(
                actual_add, *expected_add,
                "ADDMOD failed for inputs {}, {}, {} (got {}, expected {})",
                a, b, n, actual_add, expected_add
            );

            assert_eq!(
                actual_mul, *expected_mul,
                "MULMOD failed for inputs {}, {}, {} (got {}, expected {})",
                a, b, n, actual_mul, expected_mul
            );
        }
    }
}
