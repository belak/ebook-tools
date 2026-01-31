//! CLI command structure for ebook-drm.

use anyhow::Result;
use clap::{Parser, Subcommand};

/// ebook-drm: DRM removal tool for ebooks
#[derive(Parser, Debug)]
#[command(name = "ebook-drm")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Remove DRM from ebook files (auto-detect format)
    Clean {
        /// Input file path
        input: String,

        /// Output file path (optional, defaults to input with -nodrm suffix)
        output: Option<String>,
    },
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Clean { input, output } => {
                println!("TODO: Remove DRM from: {}", input);
                if let Some(out) = output {
                    println!("Output to: {}", out);
                }
                Ok(())
            }
        }
    }
}
