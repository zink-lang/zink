//! Zink testing framework.
#![deny(missing_docs)]

mod api;
mod bytes;
mod contract;
mod evm;
mod result;

pub use self::{
    bytes::Bytes32,
    contract::Contract,
    evm::{Info, EVM},
    result::Result,
};
pub use hex;
pub use revm::primitives::{Halt, OutOfGasError, U256};
pub use tracing as log;
pub use zabi::selector::keccak256;

/// Set up the logger.
pub fn setup_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .without_time()
        .compact()
        .try_init()
        .ok();
}
