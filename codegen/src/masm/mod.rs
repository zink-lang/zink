//! MacroAssembler used by the code generation.

use crate::{abi::ToLSBytes, asm::Assembler, Error, Result};
use std::ops::{Deref, DerefMut};
use wasmparser::{Ieee32, Ieee64};

/// EVM MacroAssembler.
pub struct MacroAssembler {
    /// Low level assembler.
    pub asm: Assembler,
}

impl Deref for MacroAssembler {
    type Target = Assembler;

    fn deref(&self) -> &Self::Target {
        &self.asm
    }
}

impl DerefMut for MacroAssembler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asm
    }
}

impl MacroAssembler {
    /// New macro assembler.
    pub fn new(params_count: u8) -> Result<Self> {
        let is_main = params_count == 0;

        // Build the low level assembler
        let asm = if is_main {
            Assembler::new(0)
        } else {
            // STACK: PC + [..params]
            Assembler::new(params_count + 1)
        };

        // Build the macro assembler
        let mut masm = Self { asm };
        if !is_main {
            masm._jumpdest()?;
            masm.shift_pc(params_count, true)?;
        }

        Ok(masm)
    }

    /// Store data in memory.
    pub fn memory_write(&mut self, ty: impl ToLSBytes) -> Result<()> {
        // use the current memory pointer as offset
        // to store the data.
        let offset = self.mp.to_ls_bytes();
        self.push(&offset)?;
        self._mstore()?;

        // mock the memory usages.
        let value = ty.to_ls_bytes();
        self.increment_mp(value.as_ref().len() as u8)?;

        // post logic for memory write, leave the
        // data size and memory offset on the stack.
        self.push(value.as_ref())?; // push value
        self.push(&offset)?; // push offset

        Ok(())
    }

    /// Get the current program counter offset.
    pub fn pc_offset(&self) -> u16 {
        self.asm.buffer().len() as u16
    }

    /// Place n bytes on stack.
    pub fn push(&mut self, bytes: &[u8]) -> Result<()> {
        let len = bytes.len();
        match len {
            0 => self.asm._push0(),
            1 => self.asm._push1(),
            2 => self.asm._push2(),
            3 => self.asm._push3(),
            4 => self.asm._push4(),
            5 => self.asm._push5(),
            6 => self.asm._push6(),
            7 => self.asm._push7(),
            8 => self.asm._push8(),
            9 => self.asm._push9(),
            10 => self.asm._push10(),
            11 => self.asm._push11(),
            12 => self.asm._push12(),
            13 => self.asm._push13(),
            14 => self.asm._push14(),
            15 => self.asm._push15(),
            16 => self.asm._push16(),
            17 => self.asm._push17(),
            18 => self.asm._push18(),
            19 => self.asm._push19(),
            20 => self.asm._push20(),
            21 => self.asm._push21(),
            22 => self.asm._push22(),
            23 => self.asm._push23(),
            24 => self.asm._push24(),
            25 => self.asm._push25(),
            26 => self.asm._push26(),
            27 => self.asm._push27(),
            28 => self.asm._push28(),
            29 => self.asm._push29(),
            30 => self.asm._push30(),
            31 => self.asm._push31(),
            32 => self.asm._push32(),
            _ => return Err(Error::StackIndexOutOfRange(len as u8)),
        }?;

        self.asm.emitn(bytes);
        Ok(())
    }

    /// Swap memory by target index.
    pub fn swap(&mut self, index: u8) -> Result<()> {
        match index {
            0 => Ok(()),
            1 => self.asm._swap1(),
            2 => self.asm._swap2(),
            3 => self.asm._swap3(),
            4 => self.asm._swap4(),
            5 => self.asm._swap5(),
            6 => self.asm._swap6(),
            7 => self.asm._swap7(),
            8 => self.asm._swap8(),
            9 => self.asm._swap9(),
            10 => self.asm._swap10(),
            11 => self.asm._swap11(),
            12 => self.asm._swap12(),
            13 => self.asm._swap13(),
            14 => self.asm._swap14(),
            15 => self.asm._swap15(),
            16 => self.asm._swap16(),
            _ => Err(Error::StackIndexOutOfRange(index)),
        }
    }

    /// Shift the program counter to the bottom or the top of the
    /// parameters. This is used by the callee function for jumping
    /// back to the caller function.
    pub fn shift_pc(&mut self, count: u8, from_top: bool) -> Result<()> {
        self.swap(count)?;

        if from_top {
            if count > 1 {
                return self.shift_pc(count - 1, from_top);
            }
        } else {
            // TODO: Optimize the shift logic when params lg 2.
            //
            // 3 means two swaps, base gas cost is 6, which means
            // using DUP will be cheaper: DUPN + POP = 3 + 2 = 5
            // in total.
            //
            // if count > 2 {}
            if count > 0 {
                return self.shift_pc(count - 1, from_top);
            }
        }

        Ok(())
    }

    /// Return zero or more values from the function.
    ///
    /// The return instruction is a shortcut for an unconditional
    /// branch to the outermost block, which implicitly is the body
    /// of the current function.
    ///
    /// NOTE: This `return` could be different from the `return` in
    /// the EVM.
    pub fn _return(&mut self) -> Result<()> {
        todo!()
    }

    /// Push a 32-bit integer value on the stack.
    pub fn _i32_const(&mut self, _value: i32) -> Result<()> {
        todo!()
    }

    /// Push a 64-bit integer value on the stack.
    pub fn _i64_const(&mut self, _value: i64) -> Result<()> {
        todo!()
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
}
