// Signed Integer instructions

use crate::{MacroAssembler, Result};

impl MacroAssembler {
    /// sign-agnostic rotate left
    ///
    /// Return the result of rotating i1 left by k bits.
    pub fn _rotl(&mut self) -> Result<()> {
        todo!()
    }

    /// sign-agnostic rotate right
    ///
    /// Return the result of rotating i1 right by k bits.
    pub fn _rotr(&mut self) -> Result<()> {
        todo!()
    }

    /// sign-agnostic count leading zero bits
    ///
    /// Return the number of leading zero bits in i, all zero bits
    /// are considered leading if the value is zero.
    pub fn _clz(&mut self) -> Result<()> {
        todo!()
    }

    /// sign-agnostic count leading zero bits
    ///
    /// Return the number of leading zero bits in i, all zero bits
    /// are considered trailing if the value is zero.
    pub fn _ctz(&mut self) -> Result<()> {
        todo!()
    }

    /// sign-agnostic count number of one bits
    ///
    /// Return the count of no zero bits in i.
    pub fn _popcnt(&mut self) -> Result<()> {
        todo!()
    }

    /// Truncate a 32-bit float to an integer
    pub fn _trunc_f32(&mut self) -> Result<()> {
        todo!()
    }

    /// Truncate a 64-bit float to an integer
    pub fn _trunc_f64(&mut self) -> Result<()> {
        todo!()
    }
}
