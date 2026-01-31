//! Error types for the ebook-drm library.
//!
//! At the moment, this just exports anyhow types, but in the future it may be a more carefully
//! scoped thiserror setup. The CLI just uses anyhow directly.

pub use anyhow::{Error, Result};
