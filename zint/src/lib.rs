//! Zink testing framework.
#![deny(missing_docs)]

mod bytes;
mod contract;
mod ethers;
mod evm;
mod result;

pub use self::{
    bytes::Bytes32,
    contract::Contract,
    ethers::Ethers,
    evm::{Info, InstructionResult, EVM, U256},
    result::Result,
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
