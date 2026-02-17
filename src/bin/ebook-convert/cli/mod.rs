use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;

/// ebook-convert: Convert ebooks between formats.
#[derive(Parser, Debug)]
#[command(name = "ebook-convert")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Input file path.
    input: PathBuf,

    /// Output file path (optional — derived from input + target format if omitted).
    output: Option<PathBuf>,

    /// Target format (epub, kepub, mobi, azw3).
    #[arg(short, long)]
    to: Option<ebook_tools::Format>,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        let input_format = ebook_tools::Format::from_path(&self.input);

        // Determine the target format: --to flag first, then output extension.
        let target_format = if let Some(fmt) = self.to {
            Some(fmt)
        } else {
            self.output
                .as_ref()
                .and_then(|p| ebook_tools::Format::from_path(p))
        };

        let target_format = match target_format {
            Some(fmt) => fmt,
            None => {
                bail!(
                    "Cannot determine target format. Use --to or provide an output path with a recognized extension."
                );
            }
        };

        // Derive output path if not provided.
        let output = self.output.unwrap_or_else(|| {
            let stem = self.input.file_stem().unwrap_or_default();
            self.input.with_file_name(format!(
                "{}.{}",
                stem.to_string_lossy(),
                target_format.extension()
            ))
        });

        match input_format {
            Some(fmt) => {
                println!("Input:  {} ({fmt})", self.input.display());
                println!("Output: {} ({target_format})", output.display());
                println!();
                println!("TODO: Convert ebook");
            }
            None => {
                bail!("Unknown input format: {}", self.input.display());
            }
        }

        Ok(())
    }
}
