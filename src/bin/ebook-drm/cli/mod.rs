//! CLI command structure for ebook-drm.

use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

/// ebook-drm: DRM removal tool for ebooks
#[derive(Parser, Debug)]
#[command(name = "ebook-drm")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Remove DRM from ebook files (auto-detect format)
    Clean {
        /// Input file path
        input: PathBuf,

        /// Output file path (optional, defaults to input with -nodrm suffix)
        output: Option<PathBuf>,
    },
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Clean { input, output } => {
                let format = ebook_tools::Format::from_path(&input);
                match format {
                    Some(fmt) => {
                        println!("File:   {}", input.display());
                        println!("Format: {fmt}");
                        if let Some(out) = output {
                            println!("Output: {}", out.display());
                        }
                        println!();
                        println!("TODO: Remove DRM from ebook");
                    }
                    None => {
                        bail!("Unknown ebook format: {}", input.display());
                    }
                }
                Ok(())
            }
        }
    }
}
