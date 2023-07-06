//! Pre-visitor for parsing WASM.

use anyhow::Result;
use wasmparser::{Operator, VisitOperator};

use crate::CodeGen;

/// A pre-visitor that validates the WASM and then visits it.
pub struct ValidateThenVisit<'a, T, U>(pub T, pub &'a mut U);

macro_rules! validate_then_visit {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
                self.0.$visit($($($arg.clone()),*)?)?;
                // Only visit operators if the compiler is in a reachable code state. If
                // the compiler is in an unrechable code state, most of the operators are
                // ignored except for If, Block, Loop, Else and End. These operators need
                // to be observed in order to keep the control stack frames balanced and to
                // determine if reachability should be restored.
                let visit_when_unreachable = visit_op_when_unreachable(Operator::$op $({ $($arg: $arg.clone()),* })?);
                if true || visit_when_unreachable  {
                    Ok(self.1.$visit($($($arg),*)?))
                } else {
                    Ok(U::Output::default())
                }
            }
        )*
    };
}

fn visit_op_when_unreachable(op: Operator) -> bool {
    use Operator::*;
    match op {
        If { .. } | Block { .. } | Loop { .. } | Else | End => true,
        _ => false,
    }
}

/// Trait to handle reachability state.
trait ReachableState {
    /// Returns true if the current state of the program is reachable.
    fn is_reachable(&self) -> bool;
}

impl ReachableState for CodeGen {
    fn is_reachable(&self) -> bool {
        true
    }
}

impl<'a, T, U> VisitOperator<'a> for ValidateThenVisit<'_, T, U>
where
    T: VisitOperator<'a, Output = wasmparser::Result<()>>,
    U: VisitOperator<'a> + ReachableState,
    U::Output: Default,
{
    type Output = Result<U::Output>;

    wasmparser::for_each_operator!(validate_then_visit);
}
