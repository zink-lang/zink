//! Conta Configuration
use serde::{Deserialize, Serialize};

/// Conta configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The packages should be kept in order by the
    /// dependency graph.
    pub packages: Vec<String>,
}
