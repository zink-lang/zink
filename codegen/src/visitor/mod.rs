//! This module is the central place for machine code emission.
//!
//! It defines an implementation of wasmparser's Visitor trait for
//! `Function`; which defines a visitor per op-code, which validates
//! and dispatches to the corresponding machine code emitter.

use crate::{Function, Result};
use paste::paste;
use tracing::trace;
use wasmparser::{for_each_operator, BlockType, BrTable, Ieee32, Ieee64, MemArg, VisitOperator};

mod call;
mod control;
mod local;
mod log;

/// A macro to define unsupported WebAssembly operators.
///
/// This macro calls itself recursively;
/// 1. It no-ops when matching a supported operator.
/// 2. Defines the visitor function and panics when
///    matching an unsupported operator.
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
macro_rules! map_wasm_operators {
    (@basic $ty:tt, $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        paste! {
            fn [< visit_ $ty _ $wasm >](&mut self $(,$arg: $argty),*) -> Self::Output {
                trace!("{}.{}", stringify!($ty), stringify!($evm));

                let before = self.masm.buffer().len();
                self.masm.[< _ $evm >]()?;

                let instr = self.masm.buffer()[before..].to_vec();
                self.backtrace.push(instr);

                Ok(())
            }
        }
    };
    (@integer32 $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@basic i32, $wasm, $evm $($arg: $argty),*);
    };
    (@integer64 $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@basic i64, $wasm, $evm $($arg: $argty),*);
    };
    (@integer $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@integer32 $wasm, $evm $($arg: $argty),*);
        map_wasm_operators!(@integer64 $wasm, $evm $($arg: $argty),*);
    };
    (@xdr $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        paste!{
            map_wasm_operators!(@integer [< $wasm _s >], $evm $($arg: $argty),*);
            map_wasm_operators!(@integer [< $wasm _u >], $evm $($arg: $argty),*);
        }
    };
    (@float32 $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@basic f32, $wasm, $evm $($arg: $argty),*);
    };
    (@float64 $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@basic f64, $wasm, $evm $($arg: $argty),*);
    };
    (@float $wasm:tt, $evm:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@float32 $wasm, $evm $($arg: $argty),*);
        map_wasm_operators!(@float64 $wasm, $evm $($arg: $argty),*);
    };
    (@integer_and_float $op:tt $($arg:ident: $argty:ty),*) => {
        map_wasm_operators!(@integer $op, $op);
        map_wasm_operators!(@float $op, $op);
    };
    (@field ($($field:ident).*) ($op:tt -> $evm:tt) $($arg:tt: $argty:ty),* ) => {
        paste! {
            fn [< visit_ $op >](&mut self, $($arg: $argty),*) -> Self::Output {
                let mut log = stringify!($op).to_string();
                log = log.replace('_', ".");

                $(
                    let fmt = &format!(" {:?}", $arg);
                    if fmt != " Empty" {
                        log.push_str(&fmt);
                    }
                )*

                trace!("{}", log);

                let before = self.masm.buffer().len();
                self.$($field.)*[< _ $evm >]($($arg),*)?;

                let instr = self.masm.buffer()[before..].to_vec();
                self.backtrace.push(instr);
                Ok(())
            }
        }
    };
    (
        all: [$($all:tt),+],
        xdr: [$($xdr:tt),+],
        integer: [$($integer:tt),+],
        integer_and_float: [$($op:tt),+],
        float: [$($float:tt),+],
        map: {
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
        masm: {
            $( $masm:tt $(: { $($marg:ident: $margty:ty),+ })? ),+
        },
        global: {
            $( $global:tt $(: { $($garg:ident: $gargty:ty),+ })? ),+
        }
    ) => {
        paste! {
            $(map_wasm_operators!(@integer_and_float $op);)+

            $(
                map_wasm_operators!(@integer [< $all _s >], [< s $all >]);
                map_wasm_operators!(@integer [< $all _u >], $all);
                map_wasm_operators!(@float $all, $all);
            )+

            $(map_wasm_operators!(@integer $integer, $integer);)+
            $(map_wasm_operators!(@xdr $xdr, $xdr);)+
            $(map_wasm_operators!(@float $float, $float);)+

            $(
                map_wasm_operators!(@integer [< $map_int_wasm _s >], [< s $map_int_evm >]);
                map_wasm_operators!(@integer [< $map_int_wasm _u >], $map_int_evm);
            )+

            $(
                map_wasm_operators!(@integer $mem, $mem _arg: MemArg);
                map_wasm_operators!(@float $mem, $mem _arg: MemArg);
            )+


            $(
                map_wasm_operators!(@xdr $mem_integer, $mem_integer _arg: MemArg);
            )+

            $(
                map_wasm_operators!(@integer64 [< $mem_integer64 _s >], $mem_integer64 _arg: MemArg );
                map_wasm_operators!(@integer64 [< $mem_integer64 _u >], $mem_integer64 _arg: MemArg );
            )+


            $(
                map_wasm_operators!(@integer $mem_signed, $mem_signed _arg: MemArg);
            )+

            $(
                map_wasm_operators!(@integer $mem_signed_and_float, $mem_signed_and_float _arg: MemArg);
                map_wasm_operators!(@float $mem_signed_and_float, $mem_signed_and_float _arg: MemArg);
            )+

            $(
                map_wasm_operators!(@integer64 $mem_signed64, $mem_signed64 _arg: MemArg);
            )+

            $(
                map_wasm_operators!(@field (masm) ($masm -> $masm) $( $($marg: $margty),+ )?);
            )+

            $(
                map_wasm_operators!(@field () ($global -> $global) $( $($garg: $gargty),+ )?);
            )+
        }
    };
}

impl VisitOperator<'_> for Function {
    type Output = Result<()>;

    for_each_operator!(impl_visit_operator);

    map_wasm_operators! {
        all: [div, lt, gt, ge, le],
        xdr: [shr, trunc_f32, trunc_f64],
        integer: [and, clz, ctz, eqz, or, popcnt, rotl, rotr, shl, xor],
        integer_and_float: [add, sub, mul, eq, ne],
        float: [
            abs, ceil, copysign, floor, max, min, nearest, neg, sqrt,
            convert_i32_s, convert_i32_u, convert_i64_s, convert_i64_u,
            trunc
        ],
        map: {
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
        masm: {
            drop,
            memory_grow: {
                mem: u32,
                mem_byte: u8
            },
            memory_size: {
                mem: u32,
                mem_byte: u8
            },
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
            f64_reinterpret_i64
        },
        global: {
            else, select, end, nop, unreachable,
            if: {
                blockty: BlockType
            },
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
            local_get: {
                local_index: u32
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
            call: {
                func_index: u32
            },
            call_indirect: {
                type_index: u32,
                table_index: u32,
                table_byte: u8
            }
        }
    }

    // Custom implementation for the return instruction
    fn visit_return(&mut self) -> Self::Output {
        trace!("return");

        let before = self.masm.buffer().len();

        // for early returns in a function, emit return code with value 1 (true)
        if self.is_main || self.abi.is_some() {
            tracing::trace!("early return from main function");
            self.masm.emit_return_value(&[1])?;
        } else {
            tracing::trace!("early return from call");
            self.masm.call_return(self.ty.results())?;
        }

        let instr = self.masm.buffer()[before..].to_vec();
        self.backtrace.push(instr);

        Ok(())
    }
}
