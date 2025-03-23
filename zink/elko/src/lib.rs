//! Zink package manager.

mod build;
pub mod examples;
mod new;
pub mod utils;

pub use self::{build::Build, new::New};
pub use zinkc::cli::Compile;
