//! This module is the central place for machine code emission.
//!
//! It defines an implementation of wasmparser's Visitor trait for
//! `CodeGen`; which defines a visitor per op-code, which validates
//! and dispatches to the corresponding machine code emitter.

use crate::CodeGen;
use wasmparser::{for_each_operator, VisitOperator};

macro_rules! define_visit_operator {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
                println!("visit operator: {}", stringify!($op))
            }
        )*
    }
}

impl<'a> VisitOperator<'a> for CodeGen {
    type Output = ();

    // fn visit_i32_add(&mut self) -> Self::Output {
    //     // self.masm.add()
    // }

    for_each_operator!(define_visit_operator);
}
