//! Zink compiler configuration.

/// Zink compiler configuration.
#[derive(Default)]
pub struct Config {
    /// If enable dispatcher.
    pub dispatcher: bool,
    /// If enable constructor.
    pub constructor: bool,
}
