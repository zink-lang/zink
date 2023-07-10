//! This module is the central place for machine code emission.
//!
//! It defines an implementation of wasmparser's Visitor trait for
//! `CodeGen`; which defines a visitor per op-code, which validates
//! and dispatches to the corresponding machine code emitter.

use crate::{
    // abi::Type,
    // control::{ControlStackFrame, ControlStackFrameType},
    CodeGen,
    Result,
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
    ( @mvp End => visit_end $($rest:tt)* ) => {
        fn visit_end(&mut self) -> Self::Output {
            trace!("end");

            // TODO:
            //
            // 1. check the stack output of the current context
            // 2. check the stack availability of the parent context
            // 3. pop the stack
            //
            // Otherwise we return all of the data from stack.
            self.masm.asm.push(1)?;    // PUSH1
            self.masm.emit(0);     // 0x00
            self.masm.mstore();   // MSTORE

            // Return from the stored memory.
            //
            // TODO:
            //
            // 1. get size from function signature
            self.masm.asm.push(1)?;     // PUSH1
            self.masm.emit(32);     // 0x32 - 1 stack item
            self.masm.asm.push(1)?;     // PUSH1
            self.masm.emit(0);      // 0x00  - from 0
            self.masm.ret();       // RET
            Ok(())
        }

        impl_visit_operator!($($rest)*);
    };
    ( @mvp LocalGet { local_index: u32 } => visit_local_get $($rest:tt)* ) => {
        fn visit_local_get(&mut self, local_index: u32) -> Self::Output {
            trace!("local.get {}", local_index);

            if (local_index as usize) < self.env.params().len() {
                // The size of the data index is always 32 bytes.
                self.masm.push(32)?;
                self.masm.calldata_load();
            } else {
                todo!("local.get {}", local_index);
            }

            Ok(())
        }

        impl_visit_operator!($($rest)*);
    };
    ( @mvp I32Add => visit_i32_add $($rest:tt)* ) => {
        fn visit_i32_add(&mut self) -> Self::Output {
            trace!("i32.add");
            self.masm.asm.add()?;
            Ok(())
        }

        impl_visit_operator!($($rest)*);
    };
    // ( @mvp If { blockty: $crate::BlockType } => visit_if $($rest:tt)* ) => {
    //     fn visit_if(&mut self, _blockty: wasmparser::BlockType) -> Self::Output {
    //         trace!("If");
    //
    //         // let frame = ControlStackFrame::new(ControlStackFrameType::If, self.masm.pc_offset(), blockty);
    //         // self.masm.push(1)?;             // PUSH1
    //         // self.masm.data(&frame.label()); // The byte offset of the counter of the destination.
    //         // self.control.push(frame.align()?);
    //         // self.masm.jumpi();
    //
    //         Ok(())
    //     }
    //
    //     impl_visit_operator!($($rest)*);
    // };
    // ( @mvp End => visit_end $($rest:tt)* ) => {
    //     fn visit_end(&mut self) -> Self::Output {
    //         trace!("end");
    //
    //         // let frame = self.control.pop()?;
    //         // let mut pc = frame.pc_offset() - 3;
    //         // // self.masm.push();
    //         // let label = frame.label();
    //
    //     }
    //
    //     impl_visit_operator!($($rest)*);
    // };
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
}
