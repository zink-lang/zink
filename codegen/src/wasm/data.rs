//! Dataset in code generation

use crate::{Error, Result};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

/// Data section conversion
///
/// NOTE: current only support constant expression.
#[derive(Default, Clone, Debug)]
pub struct Data(BTreeMap<i32, Vec<u8>>);

impl Data {
    /// Load data from offset and size
    pub fn load(&self, offset: i32, size: usize) -> Result<Vec<u8>> {
        for ptr in self.0.keys().cloned().rev() {
            if offset >= ptr {
                let start = (offset - ptr) as usize;
                let data = self.get(&ptr).ok_or(Error::DataNotFound(offset, size))?;

                return Ok(data[start..start + size].to_vec());
            }
        }

        Err(Error::DataNotFound(offset, size))
    }
}

impl Deref for Data {
    type Target = BTreeMap<i32, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
