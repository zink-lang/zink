//! Zink compiler configuration.

#[cfg(feature = "cli")]
use ccli::clap;

/// Zink compiler configuration.
#[derive(Debug, Default)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
pub struct Config {
    /// If enable dispatcher.
    pub dispatcher: bool,
    /// If enable constructor.
    pub constructor: bool,
}

impl Config {
    /// With dispatcher value.
    pub fn dispatcher(mut self, dispatcher: bool) -> Self {
        self.dispatcher = dispatcher;
        self
    }

    /// With constructor value.
    pub fn constructor(mut self, constructor: bool) -> Self {
        self.constructor = constructor;
        self
    }
}
