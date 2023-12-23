//! Code generators
//!
//! - CONSTRUCTOR
//! - DISPATCHER
//! - FUNCTION
//! - CODE

mod code;
mod constructor;
mod dispatcher;
mod func;

pub use self::{
    code::{Code, ExtFunc},
    constructor::Constructor,
    dispatcher::Dispatcher,
    func::Function,
};
