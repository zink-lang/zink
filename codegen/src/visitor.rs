//! This module is the central place for machine code emission.
//!
//! It defines an implementation of wasmparser's Visitor trait for
//! `CodeGen`; which defines a visitor per op-code, which validates
//! and dispatches to the corresponding machine code emitter.

use crate::{
    abi::{Offset, Type},
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
    ( @mvp End => visit_end $($rest:tt)* ) => {
        /// Handle instruction end for different situations.
        ///
        /// TODO: (#28)
        ///
        /// - End of control flow instructions.
        /// - End of function.
        /// - End of program.
        fn visit_end(&mut self) -> Self::Output {
            let offset = self.masm.mstore()?.offset();
            let size = self.env.results().align().offset();

            self.masm.asm.push(&size)?;
            self.masm.asm.push(&offset)?;
            self.masm.ret()?;

            trace!("end");
            Ok(())
        }

        impl_visit_operator!($($rest)*);
    };
    ( @mvp LocalGet { local_index: u32 } => visit_local_get $($rest:tt)* ) => {
        fn visit_local_get(&mut self, local_index: u32) -> Self::Output {
            trace!("local.get {}", local_index);

            if (local_index as usize) < self.env.params().len() {
                self.masm.push(&self.locals[local_index as usize].offset())?;
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
    ( @mvp If { blockty: $crate::BlockType } => visit_if $($rest:tt)* ) => {
        fn visit_if(&mut self, blockty: wasmparser::BlockType) -> Self::Output {
            trace!("If");

            let _frame = ControlStackFrame::new(ControlStackFrameType::If, self.masm.pc_offset(), blockty);
            // self.masm.push(1)?;             // PUSH1
            // self.masm.data(&frame.label()); // The byte offset of the counter of the destination.
            // self.control.push(frame.align()?);
            // self.masm.jumpi();

            Ok(())
        }

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
}
