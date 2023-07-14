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
use wasmparser::{for_each_operator, BlockType, Ieee32, Ieee64, MemArg, VisitOperator};

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

    fn visit_unreachable(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_nop(&mut self) -> Self::Output {
        // Perform nothing.
        Ok(())
    }
    fn visit_block(&mut self, _: BlockType) -> Self::Output {
        todo!()
    }
    fn visit_loop(&mut self, _: BlockType) -> Self::Output {
        todo!()
    }
    fn visit_else(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_br(&mut self, _: u32) -> Self::Output {
        todo!()
    }
    fn visit_br_if(&mut self, _: u32) -> Self::Output {
        todo!()
    }
    fn visit_br_table(&mut self, _: wasmparser::BrTable<'a>) -> Self::Output {
        todo!()
    }
    fn visit_return(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_call_indirect(&mut self, _: u32, _: u32, _: u8) -> Self::Output {
        todo!()
    }
    fn visit_drop(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_select(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_local_set(&mut self, _: u32) -> Self::Output {
        todo!()
    }
    fn visit_local_tee(&mut self, _: u32) -> Self::Output {
        todo!()
    }
    fn visit_global_get(&mut self, _: u32) -> Self::Output {
        todo!()
    }
    fn visit_global_set(&mut self, _: u32) -> Self::Output {
        todo!()
    }
    fn visit_i32_load(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_f32_load(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_f64_load(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_load8_s(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_load8_u(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_load16_s(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_load16_u(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load8_s(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load8_u(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load16_s(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load16_u(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load32_s(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_load32_u(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_store(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_store(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_f32_store(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_f64_store(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_store8(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i32_store16(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_store8(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_store16(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_i64_store32(&mut self, _: MemArg) -> Self::Output {
        todo!()
    }
    fn visit_memory_size(&mut self, _: u32, _: u8) -> Self::Output {
        todo!()
    }
    fn visit_memory_grow(&mut self, _: u32, _: u8) -> Self::Output {
        todo!()
    }
    fn visit_i32_const(&mut self, _: i32) -> Self::Output {
        todo!()
    }
    fn visit_i64_const(&mut self, _: i64) -> Self::Output {
        todo!()
    }
    fn visit_f32_const(&mut self, _: Ieee32) -> Self::Output {
        todo!()
    }
    fn visit_f64_const(&mut self, _: Ieee64) -> Self::Output {
        todo!()
    }
    fn visit_i32_eqz(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_ne(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_eqz(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_ne(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_ne(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_ne(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_clz(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_ctz(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_popcnt(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_rem_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_rem_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_and(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_rotl(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_rotr(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_clz(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_ctz(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_popcnt(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_rem_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_rem_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_and(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_rotl(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_rotr(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_abs(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_neg(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_ceil(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_floor(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_trunc(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_nearest(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_sqrt(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_min(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_max(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_copysign(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_abs(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_neg(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_ceil(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_floor(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_trunc(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_nearest(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_sqrt(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_min(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_max(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_copysign(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_wrap_i64(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_trunc_f32_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_trunc_f32_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_trunc_f64_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_trunc_f64_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_extend_i32_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_extend_i32_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_trunc_f32_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_trunc_f32_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_trunc_f64_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_trunc_f64_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_convert_i32_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_convert_i32_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_convert_i64_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_convert_i64_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_demote_f64(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_convert_i32_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_convert_i32_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_convert_i64_s(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_convert_i64_u(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_promote_f32(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i32_reinterpret_f32(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_i64_reinterpret_f64(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f32_reinterpret_i32(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_reinterpret_i64(&mut self) -> Self::Output {
        todo!()
    }

    impl_arithmetic_ops! {
        @common[add, sub, mul, eq],
        @xdr[div, lt, gt],
        @signed[or, xor, shl],
        @integer[shr],
        @map[ge => sgt, le => slt],
    }
}
