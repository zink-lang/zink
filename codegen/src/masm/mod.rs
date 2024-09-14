//! MacroAssembler used by the code generation.

use crate::{
    asm::Assembler,
    wasm::{ToLSBytes, Type},
    Error, Result,
};
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

mod cmp;
mod embed;
mod float;
mod integer;
mod memory;
mod ret;
mod stack;

/// EVM MacroAssembler.
#[derive(Default, Debug, Clone)]
pub struct MacroAssembler {
    /// Low level assembler.
    pub(crate) asm: Assembler,
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

/// Info for memory position.
pub struct MemoryInfo {
    /// Memory offset.
    pub offset: SmallVec<[u8; 8]>,

    /// Memory size
    pub size: usize,
}

impl MacroAssembler {
    /// Store data in memory with at current memory byte pointer.
    pub fn memory_write(&mut self, ty: impl Type) -> Result<MemoryInfo> {
        let offset = self.mp.to_ls_bytes();

        // mock the memory usages.
        let size = ty.align();
        self.increment_mp(size)?;

        // write memory
        self.memory_write_at(&offset)?;
        Ok(MemoryInfo { offset, size })
    }

    /// Write bytes to memory.
    pub fn memory_write_bytes(&mut self, bytes: &[u8]) -> Result<MemoryInfo> {
        let len = bytes.len();

        // TODO: if len is out of 32.
        self.push(bytes)?;
        self.memory_write(len)
    }

    /// Store data in memory at offset.
    ///
    /// Returns the size in the lowest significant bytes.
    pub fn memory_write_at(&mut self, offset: &[u8]) -> Result<()> {
        self.push(offset)?;
        self._mstore()?;

        Ok(())
    }

    /// Get the current program counter offset.
    pub fn pc_offset(&self) -> u16 {
        self.asm.buffer().len() as u16
    }

    /// Place n bytes on stack.
    pub fn push(&mut self, bytes: &[u8]) -> Result<()> {
        tracing::trace!("push bytes: 0x{:x?}", bytes);
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

    /// Get byte offset of the memory pointer.
    pub fn mp_offset<F>(&self, f: F) -> Result<SmallVec<[u8; 8]>>
    where
        F: Fn(usize) -> Result<usize>,
    {
        Ok(f(self.mp)?.to_ls_bytes())
    }

    /// Get the stack pointer.
    pub fn sp(&self) -> u8 {
        self.asm.sp
    }

    /// Swap memory by target index.
    pub fn swap(&mut self, index: u8) -> Result<()> {
        tracing::trace!("swap index: {}", index);
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

    /// Duplicate stack item by target index.
    pub fn dup(&mut self, index: u8) -> Result<()> {
        tracing::trace!("dup index: {}", index);
        match index {
            0 => Ok(()),
            1 => self.asm._dup1(),
            2 => self.asm._dup2(),
            3 => self.asm._dup3(),
            4 => self.asm._dup4(),
            5 => self.asm._dup5(),
            6 => self.asm._dup6(),
            7 => self.asm._dup7(),
            8 => self.asm._dup8(),
            9 => self.asm._dup9(),
            10 => self.asm._dup10(),
            11 => self.asm._dup11(),
            12 => self.asm._dup12(),
            13 => self.asm._dup13(),
            14 => self.asm._dup14(),
            15 => self.asm._dup15(),
            16 => self.asm._dup16(),
            _ => Err(Error::StackIndexOutOfRange(index)),
        }
    }

    /// Shift the program counter to the bottom or the top of the
    /// parameters. This is used by the callee function for jumping
    /// back to the caller function.
    pub fn shift_stack(&mut self, count: u8, from_top: bool) -> Result<()> {
        let mut swaps = 0;

        if from_top {
            swaps = count;
            while swaps > 0 {
                self.swap(swaps)?;
                swaps -= 1;
            }
        } else {
            // TODO: Optimize the shift logic when params lt 2.
            //
            // 3 means two swaps, base gas cost is 6, which means
            // using DUP will be cheaper: DUPN + POP = 3 + 2 = 5
            // in total.
            //
            // if count > 2 {}
            while swaps < count {
                swaps += 1;
                self.swap(swaps)?;
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
}
