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
use wasmparser::{for_each_operator, BlockType, BrTable, Ieee32, Ieee64, MemArg, VisitOperator};

mod control;
mod local;
mod system;

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
macro_rules! impl_wasm_instructions {
    (@basic $ty:tt, $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste! {
            fn [< visit_ $ty _ $wasm >](&mut self $($(,$arg: $argty),* )?) -> Self::Output {
                trace!("{}.{}", stringify!($ty), stringify!($op));
                self.masm.asm.[< _ $evm >]()?;

                Ok(())
            }
        }
    };
    (@integer32 $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@basic i32, $wasm, $evm $(, { $($arg: $argty),* })?);
    };
    (@integer64 $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@basic i64, $wasm, $evm $(, { $($arg: $argty),* })?);
    };
    (@signed $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@integer32 $wasm, $evm $(, { $($arg: $argty),* })?);
        impl_wasm_instructions!(@integer64 $wasm, $evm $(, { $($arg: $argty),* })?);
    };
    (@integer $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        paste!{
            impl_wasm_instructions!(@signed [< $wasm _s >], $evm $(, { $($arg: $argty),* })?);
            impl_wasm_instructions!(@signed [< $wasm _u >], $evm $(, { $($arg: $argty),* })?);
        }
    };
    (@float32 $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@basic f32, $wasm, $evm $(, { $($arg: $argty),* })?);
    };
    (@float64 $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@basic f64, $wasm, $evm $(, { $($arg: $argty),* })?);
    };
    (@float $wasm:tt, $evm:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@float32 $wasm, $evm $(, { $($arg: $argty),* })?);
        impl_wasm_instructions!(@float64 $wasm, $evm $(, { $($arg: $argty),* })?);
    };
    (@signed_and_float $op:tt $(, { $($arg:ident: $argty:ty),* })?) => {
        impl_wasm_instructions!(@signed $op, $op);
        impl_wasm_instructions!(@float $op, $op);
    };
    (@field ($($field:ident).*) $op:tt $($arg:tt: $argty:ty),* ) => {
        paste! {
            fn [< visit_ $op >](&mut self, $($arg: $argty),*) -> Self::Output {
                trace!("{}", stringify!($op));
                self.$($field.)*[< _ $op >]($($arg),*)?;

                Ok(())
            }
        }
    };
    (
        xdr: [$($xdr:tt),+],
        signed: [$($signed:tt),+],
        integer: [$($integer:tt),+],
        float: [$($float:tt),+],
        signed_and_float: [$($op:tt),+],
        map: {
            all: [$($wasm:tt => $evm:tt),+],
            integer: [$($map_int_wasm:tt => $map_int_evm:tt),+],
        },
        mem: {
            all: [$($mem:tt),+],
            integer: [$($mem_integer:tt),+],
            integer64: [$($mem_integer64:tt),+],
            signed: [$($mem_signed:tt),+],
            signed64: [$($mem_signed64:tt),+],
            signed_and_float: [$($mem_signed_and_float:tt),+],
        },
        asm: {
            $( $asm:tt $(: { $($aarg:ident: $aargty:ty),+ })? ),+
        },
        masm: {
            $( $masm:tt $(: { $($marg:ident: $margty:ty),+ })? ),+
        },
        global: {
            $( $global:tt $(: { $($garg:ident: $gargty:ty),+ })? ),+
        }
    ) => {
        paste! {
            $(impl_wasm_instructions!(@signed_and_float $op);)+

            $(
                impl_wasm_instructions!(@signed [< $xdr _s >], [< s $xdr >]);
                impl_wasm_instructions!(@signed [< $xdr _u >], $xdr);
                impl_wasm_instructions!(@float $xdr, $xdr);
            )+

            $(impl_wasm_instructions!(@signed $signed, $signed);)+
            $(impl_wasm_instructions!(@integer $integer, $integer);)+
            $(impl_wasm_instructions!(@float $float, $float);)+

            $(
                impl_wasm_instructions!(@integer $wasm, $evm);
                impl_wasm_instructions!(@float $wasm, $evm);
            )+

            $(
                impl_wasm_instructions!(@signed [< $map_int_wasm _s >], [< s $map_int_evm >]);
                impl_wasm_instructions!(@signed [< $map_int_wasm _u >], $map_int_evm);
            )+

            $(
                impl_wasm_instructions!(@signed $mem, $mem, { _arg: MemArg });
                impl_wasm_instructions!(@float $mem, $mem, { _arg: MemArg });
            )+


            $(
                impl_wasm_instructions!(@integer $mem_integer, $mem_integer, { _arg: MemArg });
            )+

            $(
                impl_wasm_instructions!(@integer64 [< $mem_integer64 _s >], $mem_integer64, { _arg: MemArg });
                impl_wasm_instructions!(@integer64 [< $mem_integer64 _u >], $mem_integer64, { _arg: MemArg });
            )+


            $(
                impl_wasm_instructions!(@signed $mem_signed, $mem_signed, { _arg: MemArg });
            )+

            $(
                impl_wasm_instructions!(@signed $mem_signed_and_float, $mem_signed_and_float, { _arg: MemArg });
                impl_wasm_instructions!(@float $mem_signed_and_float, $mem_signed_and_float, { _arg: MemArg });
            )+

            $(
                impl_wasm_instructions!(@integer64 $mem_signed64, $mem_signed64, { _arg: MemArg });
            )+

            $(
                impl_wasm_instructions!(@field (masm.asm) $asm $( $($aarg: $aargty),+ )?);
            )+

            $(
                impl_wasm_instructions!(@field (masm) $masm $( $($marg: $margty),+ )?);
            )+

            $(
                impl_wasm_instructions!(@field () $global $( $($garg: $gargty),+ )?);
            )+
        }
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

    impl_wasm_instructions! {
        xdr: [div, lt, gt],
        signed: [and, clz, ctz, eqz, or, popcnt, rotl, rotr, shl, xor],
        integer: [shr, trunc_f32, trunc_f64],
        float: [
            abs, ceil, copysign, floor, max, min, nearest, neg, sqrt,
            convert_i32_s, convert_i32_u, convert_i64_s, convert_i64_u,
            trunc
        ],
        signed_and_float: [add, sub, mul, eq, ne],
        map: {
            all: [ge => sgt, le => slt],
            integer: [rem => mod],
        },
        mem: {
            all: [load],
            integer: [load8, load16],
            integer64: [load32],
            signed: [store8, store16],
            signed64: [store32],
            signed_and_float: [store],
        },
        asm: {
            drop,
            memory_grow: {
                mem: u32,
                mem_byte: u8
            },
            memory_size: {
                mem: u32,
                mem_byte: u8
            }
        },
        masm: {
            i32_const: {
                value: i32
            },
            i64_const: {
                value: i64
            },
            f32_const: {
                value: Ieee32
            },
            f64_const: {
                value: Ieee64
            },
            i32_wrap_i64,
            i64_extend_i32_s,
            i64_extend_i32_u,
            f32_demote_f64,
            f64_promote_f32,
            i32_reinterpret_f32,
            i64_reinterpret_f64,
            f32_reinterpret_i32,
            f64_reinterpret_i64,
            return
        },
        global: {
            else, select,
            block: {
                blockty: BlockType
            },
            loop: {
                blockty: BlockType
            },
            br: {
                relative_depth: u32
            },
            br_if: {
                relative_depth: u32
            },
            br_table: {
                table: BrTable<'_>
            },
            local_set: {
                local_index: u32
            },
            local_tee: {
                local_index: u32
            },
            global_get: {
                global_index: u32
            },
            global_set: {
                global_index: u32
            },
            call_indirect: {
                type_index: u32,
                table_index: u32,
                table_byte: u8
            }
        }
    }
}
