//! Zink allocation result

/// Zink allocation error
#[derive(Debug, Clone)]
pub enum Error {
    InvalidLength,
}

/// Zink allocation result
pub type Result<T> = core::result::Result<T, Error>;
