use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;

use ebook_tools::{DrmDetector, EpubBook, Format, MetadataProvider};

/// ebook-info: Display information about an ebook file.
#[derive(Parser, Debug)]
#[command(name = "ebook-info")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the ebook file.
    file: PathBuf,

    /// Increase verbosity (-v, -vv, -vvv).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        let format = Format::from_path(&self.file);

        let Some(format) = format else {
            bail!("Unknown ebook format: {}", self.file.display());
        };

        match format {
            Format::Epub | Format::Kepub => self.show_epub()?,
            _ => {
                bail!("Unsupported format: {format}");
            }
        }

        Ok(())
    }

    fn show_epub(&self) -> Result<()> {
        let book = EpubBook::open(&self.file)?;

        let metadata = book.metadata()?;
        let drm = book.drm_status()?;

        // File info
        println!("File:      {}", self.file.display());
        let version_suffix = book
            .epub_version()
            .map(|v| format!(" {v}"))
            .unwrap_or_default();
        println!("Format:    {}{version_suffix}", book.format());
        println!("DRM:       {drm}");
        println!();

        // Metadata
        if let Some(ref title) = metadata.title {
            println!("Title:     {title}");
        }
        if !metadata.authors.is_empty() {
            println!("Authors:   {}", metadata.authors.join(", "));
        }
        if let Some(ref language) = metadata.language {
            println!("Language:  {language}");
        }
        if let Some(ref publisher) = metadata.publisher {
            println!("Publisher: {publisher}");
        }
        if let Some(ref date) = metadata.publication_date {
            println!("Date:      {date}");
        }
        if let Some(ref isbn) = metadata.isbn {
            println!("ISBN:      {isbn}");
        }
        if let Some(ref description) = metadata.description {
            // Truncate long descriptions
            let desc = if description.len() > 200 {
                format!("{}...", &description[..200])
            } else {
                description.clone()
            };
            println!("Desc:      {desc}");
        }
        if !metadata.subjects.is_empty() {
            println!("Subjects:  {}", metadata.subjects.join(", "));
        }
        if let Some(ref series) = metadata.series {
            let idx = metadata
                .series_index
                .map(|i| format!(" #{i}"))
                .unwrap_or_default();
            println!("Series:    {series}{idx}");
        }

        // Cover
        println!();
        if let Some(info) = book.cover_info() {
            println!("Cover:     Yes ({} bytes)", info.size);
        } else {
            println!("Cover:     No");
        }

        // Warnings
        let warnings = book.warnings();
        if !warnings.is_empty() {
            println!();
            println!("Warnings:");
            for w in warnings {
                println!("  - {w}");
            }
        }

        Ok(())
    }
}
