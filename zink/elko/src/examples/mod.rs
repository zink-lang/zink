//! Example templates for initializing Zink projects.
/// Structure representing a project example.
pub struct Example {
    pub lib_rs: &'static str,
    pub readme: &'static str,
}

mod addition;
mod erc20;
pub use addition::ADDITION;
pub use erc20::ERC20;
