use std::fmt;

/// The DRM status of an ebook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrmStatus {
    /// No DRM detected.
    None,
    /// DRM is present with a known scheme.
    Protected(DrmScheme),
    /// Could not determine DRM status.
    Unknown,
}

/// Known DRM schemes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrmScheme {
    AdobeAdept,
    KoboProtected,
    AmazonKindle,
    Other(String),
}

impl fmt::Display for DrmStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrmStatus::None => write!(f, "None"),
            DrmStatus::Protected(scheme) => write!(f, "Protected ({scheme})"),
            DrmStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for DrmScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrmScheme::AdobeAdept => write!(f, "Adobe ADEPT"),
            DrmScheme::KoboProtected => write!(f, "Kobo Protected"),
            DrmScheme::AmazonKindle => write!(f, "Amazon Kindle"),
            DrmScheme::Other(name) => write!(f, "{name}"),
        }
    }
}
