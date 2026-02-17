use std::path::Path;

use crate::{DrmStatus, Metadata};

/// Open a book from a file path.
pub trait BookReader {
    type Book;

    fn open(path: &Path) -> crate::Result<Self::Book>;
}

/// Read metadata from an ebook.
pub trait MetadataProvider {
    fn metadata(&self) -> crate::Result<Metadata>;
}

/// Write metadata to an ebook.
pub trait MetadataWriter {
    fn set_metadata(&mut self, metadata: &Metadata) -> crate::Result<()>;
}

/// Detect DRM status of an ebook.
pub trait DrmDetector {
    fn drm_status(&self) -> crate::Result<DrmStatus>;
}

/// Extract a cover image from an ebook.
pub trait CoverProvider {
    /// Returns the cover image as raw bytes (typically JPEG or PNG).
    fn cover(&self) -> crate::Result<Option<Vec<u8>>>;
}

/// Set or replace the cover image of an ebook.
pub trait CoverWriter {
    fn set_cover(&mut self, image_data: &[u8]) -> crate::Result<()>;
}
