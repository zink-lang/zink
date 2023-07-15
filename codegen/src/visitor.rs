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
    ($op:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_arithmetic_ops!(@signed $op, $op);
        impl_arithmetic_ops!(@float $op, $op);
    };
    (@signed64 $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste! {
            fn [< visit_i64_ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("i64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@signed $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste! {
            fn [< visit_i32_ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("i32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }

            impl_arithmetic_ops!(@signed64 $wasm, $evm $(, { $($arg: $argty),* })?);
        }
    };
    (@unsigned $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste! {
            fn [< visit_i32_ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("i32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }

            fn [< visit_i64_ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("i64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@integer $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste!{
            impl_arithmetic_ops!(@signed [< $wasm _s >], $evm);
            impl_arithmetic_ops!(@unsigned [< $wasm _u >], $evm);
        }
    };
    (@float $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste! {
            fn [< visit_f32_ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("f32.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }

            fn [< visit_f64_ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("f64.{}", stringify!($op).to_lowercase());
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@mem_integer $mem:tt) => {
        paste! {
            impl_arithmetic_ops!(@signed [< $mem _s >], $mem, { _arg: MemArg });
            impl_arithmetic_ops!(@unsigned [< $mem _u >], $mem, { _arg: MemArg });
        }
    };
    (@mem_i64 $mem:tt) => {
        paste! {
            impl_arithmetic_ops!(@signed64 [< $mem _s >], $mem, { _arg: MemArg });
            impl_arithmetic_ops!(@signed64 [< $mem _u >], $mem, { _arg: MemArg });
        }
    };
    (@mem $mem:tt) => {
        impl_arithmetic_ops!(@signed $mem, $mem, { _arg: MemArg });
        impl_arithmetic_ops!(@float $mem, $mem, { _arg: MemArg });
    };
    (@map $wasm:tt, $evm:tt) => {
        impl_arithmetic_ops!(@integer $wasm, $evm);
        impl_arithmetic_ops!(@float $wasm, $evm);
    };
    (@map_integer $wasm:tt, $evm:tt) => {
        paste! {
            impl_arithmetic_ops!(@signed [< $wasm _s >], [< s $evm >]);
            impl_arithmetic_ops!(@unsigned [< $wasm _u >], $evm);
        }
    };
    (@xdr $op:tt) => {
        paste! {
            impl_arithmetic_ops!(@map_integer $op, $op);
            impl_arithmetic_ops!(@float $op, $op);
        }
    };
    (
        @common[$($op:tt),+],
        @xdr[$($xdr:tt),+],
        @signed[$($signed:tt),+],
        @integer[$($integer:tt),+],
        @float[$($float:tt),+],
        @map[$($wasm:tt => $evm:tt),+],
        @map_integer[$($map_int_wasm:tt => $map_int_evm:tt),+],
        @mem[$($mem:tt),+],
        @mem_integer[$($mem_signed:tt),+],
        @mem_i64[$($mem_i64:tt),+],
    ) => {
        $(impl_arithmetic_ops!($op);)+
        $(impl_arithmetic_ops!(@xdr $xdr);)+
        $(impl_arithmetic_ops!(@signed $signed, $signed);)+
        $(impl_arithmetic_ops!(@integer $integer, $integer);)+
        $(impl_arithmetic_ops!(@float $float, $float);)+
        $(impl_arithmetic_ops!(@map $wasm, $evm);)+
        $(impl_arithmetic_ops!(@map_integer $map_int_wasm, $map_int_evm);)+
        $(impl_arithmetic_ops!(@mem $mem);)+
        $(impl_arithmetic_ops!(@mem_integer $mem_signed);)+
        $(impl_arithmetic_ops!(@mem_i64 $mem_i64);)+
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

    // Mark as invalid for now.
    //
    // TODO: recheck this implementation, if it is okay,
    // provide more docs.
    fn visit_unreachable(&mut self) -> Self::Output {
        self.masm._invalid()?;
        Ok(())
    }

    // Perform nothing in EVM bytecode.
    fn visit_nop(&mut self) -> Self::Output {
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
    fn visit_f32_trunc(&mut self) -> Self::Output {
        todo!()
    }
    fn visit_f64_trunc(&mut self) -> Self::Output {
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
        @common[add, sub, mul, eq, ne],
        @xdr[div, lt, gt],
        @signed[and, clz, ctz, eqz, or, popcnt, rotl, rotr, shl, xor],
        @integer[shr],
        @float[abs, ceil, copysign, floor, max, min, nearest, neg, sqrt],
        @map[ge => sgt, le => slt],
        @map_integer[rem => mod],
        @mem[load],
        @mem_integer[load8, load16],
        @mem_i64[load32],
    }
}
