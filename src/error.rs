use std::path::PathBuf;

use thiserror::Error;

use crate::Format;

/// Errors returned by ebook-core operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unsupported format: {0}")]
    UnsupportedFormat(Format),

    #[error("unknown format for path: {0}")]
    UnknownFormat(PathBuf),

    #[error("file not found: {0}")]
    FileNotFound(PathBuf),

    #[error("invalid ebook: {0}")]
    InvalidBook(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// A type alias for `Result<T, ebook_core::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
