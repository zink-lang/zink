// Integer instructions

use crate::{wasm::ToLSBytes, MacroAssembler, Result};
use wasmparser::{Ieee32, Ieee64};

impl MacroAssembler {
    /// Sub two numbers.
    pub fn _sub(&mut self) -> Result<()> {
        self._swap1()?;
        self.asm._sub()
    }

    /// Push a 32-bit integer value on the stack.
    pub fn _i32_const(&mut self, value: i32) -> Result<()> {
        if value == 0 {
            self._push0()
        } else {
            self.push(value.to_ls_bytes().as_ref())
        }
    }

    /// Push a 64-bit integer value on the stack.
    pub fn _i64_const(&mut self, value: i64) -> Result<()> {
        if value == 0 {
            self._push0()
        } else {
            self.push(value.to_ls_bytes().as_ref())
        }
    }

    /// Push a 32-bit float value on the stack.
    pub fn _f32_const(&mut self, _value: Ieee32) -> Result<()> {
        todo!()
    }

    /// Push a 64-bit float value on the stack.
    pub fn _f64_const(&mut self, _value: Ieee64) -> Result<()> {
        todo!()
    }

    /// wrap a 64-bit integer to a 32-bit integer.
    pub fn _i32_wrap_i64(&mut self) -> Result<()> {
        todo!()
    }

    /// Extend a signed 32-bit integer to a 64-bit integer.
    pub fn _i64_extend_i32_s(&mut self) -> Result<()> {
        todo!()
    }

    /// Extend an unsigned 32-bit integer to a 64-bit integer.
    pub fn _i64_extend_i32_u(&mut self) -> Result<()> {
        todo!()
    }

    /// Truncate a 64-bit float to a signed 32-bit integer.
    pub fn _f32_demote_f64(&mut self) -> Result<()> {
        todo!()
    }

    /// Truncate a 64-bit float to an unsigned 32-bit integer.
    pub fn _f64_promote_f32(&mut self) -> Result<()> {
        todo!()
    }

    /// Convert a signed 32-bit integer to a 32-bit float.
    pub fn _i32_reinterpret_f32(&mut self) -> Result<()> {
        todo!()
    }

    /// Convert a signed 64-bit integer to a 64-bit float.
    pub fn _i64_reinterpret_f64(&mut self) -> Result<()> {
        todo!()
    }

    /// Convert a 32-bit float to a signed 32-bit integer.
    pub fn _f32_reinterpret_i32(&mut self) -> Result<()> {
        todo!()
    }

    /// Convert a 64-bit float to a signed 64-bit integer.
    pub fn _f64_reinterpret_i64(&mut self) -> Result<()> {
        todo!()
    }

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
