//! This module is the central place for machine code emission.
//!
//! It defines an implementation of wasmparser's Visitor trait for
//! `CodeGen`; which defines a visitor per op-code, which validates
//! and dispatches to the corresponding machine code emitter.

use crate::{
    control::{ControlStackFrame, ControlStackFrameType},
    CodeGen, Result,
};
use paste::paste;
use tracing::trace;
use wasmparser::{for_each_operator, VisitOperator};

/// A macro to define unsupported WebAssembly operators.
///
/// This macro calls itself recursively;
/// 1. It no-ops when matching a supported operator.
/// 2. Defines the visitor function and panics when
/// matching an unsupported operator.
macro_rules! impl_visit_operator {
    ( @mvp $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident $($rest:tt)* ) => {
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

/// Implement arithmetic operators for types.
macro_rules! impl_arithmetic_ops {
    ($op:tt) => {
        paste! {
            fn [< visit_i32_ $op >](&mut self) -> Self::Output {
                trace!("i32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $op >]()?;

                Ok(())
            }

            fn [< visit_i64_ $op >](&mut self) -> Self::Output {
                trace!("i64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $op >]()?;

                Ok(())
            }

            impl_arithmetic_ops!(@float $op, $op);
        }
    };
    (@signed $wasm:tt, $evm:tt $($suffix:tt)?) => {
        paste! {
            fn [< visit_i32_ $wasm $($suffix)? >](&mut self) -> Self::Output {
                trace!("i32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }

            fn [< visit_i64_ $wasm $($suffix)? >](&mut self) -> Self::Output {
                trace!("i64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@unsigned $wasm:tt, $evm:tt $($suffix:tt)?) => {
        paste! {
            fn [< visit_i32_ $wasm $($suffix)? >](&mut self) -> Self::Output {
                trace!("i32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }

            fn [< visit_i64_ $wasm $($suffix)? >](&mut self) -> Self::Output {
                trace!("i64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@integer $wasm:tt, $evm:tt) => {
        impl_arithmetic_ops!(@signed $wasm, $evm _s);

        impl_arithmetic_ops!(@unsigned $wasm, $evm _u);
    };
    (@float $wasm:tt, $evm:tt) => {
        paste! {
            fn [< visit_f32_ $wasm >](&mut self) -> Self::Output {
                trace!("f32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }

            fn [< visit_f64_ $wasm >](&mut self) -> Self::Output {
                trace!("f64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@map $wasm:tt, $evm:tt) => {
        paste! {
            impl_arithmetic_ops!(@integer $wasm, $evm);

            impl_arithmetic_ops!(@float $wasm, $evm);
        }
    };
    (
        @common[$($op:tt),+],
        @xdr[$($xdr:tt),+],
        @signed[$($signed:tt),+],
        @integer[$($integer:tt),+],
        @map[$($wasm:tt => $evm:tt),+],
    ) => {
        $(impl_arithmetic_ops!($op);)+
        $(impl_arithmetic_ops!(@map $xdr, $xdr);)+
        $(impl_arithmetic_ops!(@signed $signed, $signed);)+
        $(impl_arithmetic_ops!(@integer $integer, $integer);)+
        $(impl_arithmetic_ops!(@map $wasm, $evm);)+
    };
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

    impl_arithmetic_ops! {
        @common[add, sub, mul, eq],
        @xdr[div, lt, gt],
        @signed[or, xor, shl],
        @integer[shr],
        @map[ge => sgt, le => slt],
    }
}
