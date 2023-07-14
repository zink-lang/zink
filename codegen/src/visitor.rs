//! This module is the central place for machine code emission.
//!
//! It defines an implementation of wasmparser's Visitor trait for
//! `CodeGen`; which defines a visitor per op-code, which validates
//! and dispatches to the corresponding machine code emitter.

use crate::{
    control::{ControlStackFrame, ControlStackFrameType},
    CodeGen, Result,
};
use tracing::trace;
use wasmparser::{for_each_operator, VisitOperator};

/// A macro to define unsupported WebAssembly operators.
///
/// This macro calls itself recursively;
/// 1. It no-ops when matching a supported operator.
/// 2. Defines the visitor function and panics when
/// matching an unsupported operator.
macro_rules! impl_visit_operator {
    ( @mvp Call { function_index: u32 } => visit_call $($rest:tt)* ) => {
        impl_visit_operator!($($rest)*);
    };
    ( @mvp End => visit_end $($rest:tt)* ) => {
        impl_visit_operator!($($rest)*);
    };
    ( @mvp LocalGet { local_index: u32 } => visit_local_get $($rest:tt)* ) => {
        impl_visit_operator!($($rest)*);
    };
    ( @mvp I32Add => visit_i32_add $($rest:tt)* ) => {
        impl_visit_operator!($($rest)*);
    };
    ( @mvp If { blockty: $crate::BlockType } => visit_if $($rest:tt)* ) => {
        impl_visit_operator!($($rest)*);
    };
    ( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident $($rest:tt)* ) => {
        fn $visit(&mut self $($(, $arg: $argty)*)?) -> Self::Output {
            trace!("{}", stringify!($op));
            Ok(())
        }

        impl_visit_operator!($($rest)*);
    };
    () => {};
}

impl<'a> VisitOperator<'a> for CodeGen {
    type Output = Result<()>;

    for_each_operator!(impl_visit_operator);

    fn visit_call(&mut self, function_index: u32) -> Self::Output {
        trace!("call {}", function_index);
        // record the current program counter and
        // pass it to the callee function.
        self.masm._pc()?;

        // register the call index to the jump table.
        self.table.call(self.masm.pc_offset(), function_index)?;

        // mock the stack output of the counter
        //
        // the program counter instructions should be relocated afterwards.
        self.masm.asm.increment_sp(1)?;
        self.masm._jump()?;
        self.masm._jumpdest()?;
        Ok(())
    }

    /// Handle instruction end for different situations.
    ///
    /// TODO: (#28)
    ///
    /// - End of control flow instructions.
    /// - End of function.
    /// - End of program.
    fn visit_end(&mut self) -> Self::Output {
        trace!("end");
        if !self.is_main {
            // TODO: handle the length of results > u8::MAX.
            self.masm.shift_pc(self.env.results().len() as u8, false)?;
            self.masm.push(&[0x04])?;
            self.masm._add()?;
            self.masm._jump()?;
            return Ok(());
        }

        // If inside a frame, pop the frame and patch
        // the program counter.
        if let Ok(frame) = self.control.pop() {
            self.table
                .label(frame.original_pc_offset, self.masm.pc_offset())?;

            // TODO: Check the stack output and make decisions
            // how to handle the results.

            // Emit JUMPDEST after at the end of the control flow.
            self.masm._jumpdest()?;
        } else {
            self.masm.memory_write(self.env.results())?;
            self.masm._return()?;
        }

        Ok(())
    }

    fn visit_local_get(&mut self, local_index: u32) -> Self::Output {
        if !self.is_main {
            return Ok(());
        }

        trace!("local.get {}", local_index);
        if (local_index as usize) < self.env.params().len() {
            self.masm
                .push(&self.locals[local_index as usize].to_ls_bytes())?;
            self.masm._calldataload()?;
        } else {
            todo!("local.get {}", local_index);
        }

        Ok(())
    }

    fn visit_i32_add(&mut self) -> Self::Output {
        trace!("i32.add");
        self.masm.asm._add()?;
        Ok(())
    }

    fn visit_if(&mut self, blockty: wasmparser::BlockType) -> Self::Output {
        trace!("If");

        // push the frame to the control stack
        let frame =
            ControlStackFrame::new(ControlStackFrameType::If, self.masm.pc_offset(), blockty);
        self.control.push(frame);

        // mock the stack output of the counter
        //
        // the program counter instructions should be patched afterwards.
        self.masm.asm.increment_sp(1)?;
        self.masm._jumpi()?;

        Ok(())
    }
}
