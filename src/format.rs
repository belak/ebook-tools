use std::fmt;
use std::path::Path;
use std::str::FromStr;

/// Supported ebook formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Epub,
    Kepub,
    Mobi,
    Azw3,
}

impl Format {
    /// Detect the format of an ebook from its file path extension.
    ///
    /// `.kepub.epub` is checked before `.epub` to avoid misidentifying KePub files.
    pub fn from_path(path: &Path) -> Option<Format> {
        let name = path.to_string_lossy().to_lowercase();

        // Check compound extensions first
        if name.ends_with(".kepub.epub") {
            return Some(Format::Kepub);
        }

        let ext = path.extension()?.to_string_lossy().to_lowercase();
        match ext.as_str() {
            "epub" => Some(Format::Epub),
            "mobi" => Some(Format::Mobi),
            "azw3" => Some(Format::Azw3),
            _ => None,
        }
    }

    /// The canonical file extension for this format (without leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Epub => "epub",
            Format::Kepub => "kepub.epub",
            Format::Mobi => "mobi",
            Format::Azw3 => "azw3",
        }
    }

    /// A human-readable name for this format.
    pub fn name(&self) -> &'static str {
        match self {
            Format::Epub => "EPUB",
            Format::Kepub => "Kobo KePub",
            Format::Mobi => "Mobipocket",
            Format::Azw3 => "Kindle AZW3",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "epub" => Ok(Format::Epub),
            "kepub" => Ok(Format::Kepub),
            "mobi" => Ok(Format::Mobi),
            "azw3" => Ok(Format::Azw3),
            _ => Err(format!("unknown format: {s}")),
        }
    }
}
