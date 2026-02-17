use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

/// ebook-edit: Edit ebook metadata and cover images.
#[derive(Parser, Debug)]
#[command(name = "ebook-edit")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Edit metadata fields of an ebook.
    Metadata {
        /// Path to the ebook file.
        file: PathBuf,

        /// Set the title.
        #[arg(long)]
        title: Option<String>,

        /// Set an author (repeatable for multiple authors).
        #[arg(long)]
        author: Vec<String>,

        /// Set the description.
        #[arg(long)]
        description: Option<String>,

        /// Set the publisher.
        #[arg(long)]
        publisher: Option<String>,

        /// Set the language.
        #[arg(long)]
        language: Option<String>,

        /// Set the ISBN.
        #[arg(long)]
        isbn: Option<String>,

        /// Set the publication date.
        #[arg(long)]
        publication_date: Option<String>,

        /// Set the series name.
        #[arg(long)]
        series: Option<String>,

        /// Set the series index.
        #[arg(long)]
        series_index: Option<f64>,
    },

    /// Manage the cover image of an ebook.
    Cover {
        #[command(subcommand)]
        action: CoverAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum CoverAction {
    /// Extract the cover image from an ebook.
    Extract {
        /// Path to the ebook file.
        file: PathBuf,

        /// Output path for the cover image.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Set the cover image of an ebook.
    Set {
        /// Path to the ebook file.
        file: PathBuf,

        /// Path to the cover image.
        image: PathBuf,
    },
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Metadata { file, .. } => {
                let format = ebook_tools::Format::from_path(&file);
                match format {
                    Some(fmt) => {
                        println!("File:   {}", file.display());
                        println!("Format: {fmt}");
                        println!();
                        println!("TODO: Write metadata to ebook");
                    }
                    None => {
                        bail!("Unknown ebook format: {}", file.display());
                    }
                }
            }
            Commands::Cover { action } => match action {
                CoverAction::Extract { file, output } => {
                    let format = ebook_tools::Format::from_path(&file);
                    match format {
                        Some(fmt) => {
                            println!("File:   {}", file.display());
                            println!("Format: {fmt}");
                            if let Some(out) = output {
                                println!("Output: {}", out.display());
                            }
                            println!();
                            println!("TODO: Extract cover image");
                        }
                        None => {
                            bail!("Unknown ebook format: {}", file.display());
                        }
                    }
                }
                CoverAction::Set { file, image } => {
                    let format = ebook_tools::Format::from_path(&file);
                    match format {
                        Some(fmt) => {
                            println!("File:   {}", file.display());
                            println!("Format: {fmt}");
                            println!("Image:  {}", image.display());
                            println!();
                            println!("TODO: Set cover image");
                        }
                        None => {
                            bail!("Unknown ebook format: {}", file.display());
                        }
                    }
                }
            },
        }

        Ok(())
    }
}
