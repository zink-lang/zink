//! Zink testing framework.
#![deny(missing_docs)]

mod bytes;
mod contract;
mod evm;

pub use self::{
    bytes::Bytes32,
    contract::Contract,
    evm::{Info, InstructionResult, EVM, U256},
};
use tracing_subscriber::EnvFilter;

/// Set up the logger.
pub fn setup_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .compact()
        .try_init()
        .ok();
}
