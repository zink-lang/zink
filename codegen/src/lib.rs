//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::{ToLSBytes, Type},
    asm::Assembler,
    codegen::CodeGen,
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    local::LocalSlot,
    masm::MacroAssembler,
    result::{Error, Result},
};
use opcodes::ShangHai as OpCode;
use smallvec::SmallVec;
use std::collections::BTreeMap;

pub mod abi;
mod asm;
mod codegen;
mod control;
mod local;
mod masm;
mod result;
mod validator;
mod visitor;

/// Maximum size of a evm bytecode in bytes.
pub const BUFFER_LIMIT: usize = 0x6000;

/// Function call labels.
pub type Labels = BTreeMap<u16, u32>;

/// Code generation buffer.
pub type Buffer = SmallVec<[u8; BUFFER_LIMIT]>;

/// Solidity's implementation uses 16 slots for locals.
/// ref: https://docs.soliditylang.org/en/v0.8.20/internals/optimizer.html#stackcompressor
pub type Locals = SmallVec<[LocalSlot; 16]>;

/// Patch program counter to buffer.
pub fn patch(buffer: &mut Buffer, original_pc: usize, target_pc: usize) -> Result<usize> {
    let mut pc = target_pc;
    let mut new_buffer: Buffer = buffer[..original_pc].into();
    let rest_buffer: Buffer = buffer[original_pc..].into();

    // Update the target program counter
    {
        // The maximum size of the PC is 2 bytes, whatever PUSH1 or PUSH2
        // takes 1 more byte.
        pc += 1;

        // Update the program counter for the edge cases.
        //
        // Start from 0xff, the lowest significant bytes of the target
        // program counter will take 2 bytes instead of 1 byte.
        //
        // | PC   | PC BYTES | TARGET PC |
        // |------|----------|-----------|
        // | 0xfe | 1        |      0xff |
        // | 0xff | 2        |     0x101 |
        pc += if pc > 0xfe {
            new_buffer.push(OpCode::PUSH2.into());
            2
        } else {
            new_buffer.push(OpCode::PUSH1.into());
            1
        }
    }

    // Check PC range.
    if pc > 0xffff {
        return Err(Error::InvalidPC(pc));
    }

    new_buffer.extend_from_slice(&pc.to_ls_bytes());
    new_buffer.extend_from_slice(&rest_buffer);

    // Check buffer size.
    if new_buffer.len() > BUFFER_LIMIT {
        return Err(Error::BufferOverflow(new_buffer.len()));
    }

    *buffer = new_buffer;
    Ok(pc)
}
