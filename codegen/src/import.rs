//! Built-in functions for EVM
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::{Error, Result};

/// EVM built-in function.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Func {
    // EVM functions.
    //
    /// Run function sload.
    Sload,
    /// Run function sstore.
    Sstore,
    /// Run function log0.
    Log0,
    /// Run function log1.
    Log1,
    /// Run function log2.
    Log2,
    /// Run function log3.
    Log3,
    /// Run function log4.
    Log4,

    //
    // Zinkc helper functions
    //
    /// Emit ABI to the compiler.
    EmitABI,
}

impl Func {
    /// Stack input size.
    pub fn stack_in(&self) -> u8 {
        match self {
            Self::Sload => 1,
            Self::Sstore => 2,
            Self::Log0 => 2,
            Self::Log1 => 4,
            Self::Log2 => 6,
            Self::Log3 => 8,
            Self::Log4 => 10,
            _ => 0,
        }
    }

    /// Stack output size.
    pub fn stack_out(&self) -> u8 {
        match self {
            Self::Sload => 1,
            Self::Sstore => 0,
            Self::Log0 => 0,
            Self::Log1 => 0,
            Self::Log2 => 0,
            Self::Log3 => 0,
            Self::Log4 => 0,
            _ => 0,
        }
    }
}

impl TryFrom<(&str, &str)> for Func {
    type Error = Error;

    fn try_from(import: (&str, &str)) -> Result<Self> {
        let (module, name) = import;
        // NOTE: `select` is not external call
        // so we don't need to check process it
        // here
        match import {
            ("evm", "sload") => Ok(Self::Sload),
            ("evm", "sstore") => Ok(Self::Sstore),
            ("evm", "log0") => Ok(Self::Log0),
            ("evm", "log1") => Ok(Self::Log1),
            ("evm", "log2") => Ok(Self::Log2),
            ("evm", "log3") => Ok(Self::Log3),
            ("evm", "log4") => Ok(Self::Log4),
            ("zinkc", "emit_abi") => Ok(Self::EmitABI),
            _ => {
                tracing::warn!("Failed to load host function: {:?}", import);
                Err(Error::HostFuncNotFound(module.into(), name.into()))
            }
        }
    }
}

/// Imported functions
#[derive(Clone, Debug, Default)]
pub struct Imports(BTreeMap<u32, Func>);

impl Imports {
    /// If the function is `emit_abi`.
    pub fn is_emit_abi(&self, index: u32) -> bool {
        self.0.get(&index) == Some(&Func::EmitABI)
    }
}

impl Deref for Imports {
    type Target = BTreeMap<u32, Func>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Imports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
