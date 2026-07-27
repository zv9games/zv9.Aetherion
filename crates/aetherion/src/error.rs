//! Error types.

use thiserror::Error;

/// Top-level Aetherion error.
#[derive(Debug, Error)]
pub enum AetherionError {
    /// Generic failure with context.
    #[error("{0}")]
    Message(String),

    /// I/O failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
