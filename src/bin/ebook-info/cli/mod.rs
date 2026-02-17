use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;

/// ebook-info: Display information about an ebook file.
#[derive(Parser, Debug)]
#[command(name = "ebook-info")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the ebook file.
    file: PathBuf,

    /// Output format: text or json.
    #[arg(short, long, default_value = "text")]
    format: OutputFormat,

    /// Increase verbosity (-v, -vv, -vvv).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        let format = ebook_tools::Format::from_path(&self.file);

        match format {
            Some(fmt) => {
                println!("File:   {}", self.file.display());
                println!("Format: {fmt}");
                println!();
                println!("TODO: Extract metadata");
                println!("TODO: Detect DRM status");
            }
            None => {
                bail!("Unknown ebook format: {}", self.file.display());
            }
        }

        Ok(())
    }
}
