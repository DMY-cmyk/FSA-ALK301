mod config;
mod assembler;
mod companion;
mod print_bundle;
mod validator;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fsa-dev-assistant")]
#[command(about = "Build pipeline for FSA ALK301 study system")]
struct Cli {
    /// Path to project root (default: parent of Dev Assistant/)
    #[arg(long, default_value = "..")]
    project_root: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Assemble HTML fragments into full pages
    Build,
    /// Validate that all expected fragment files exist
    Validate,
    /// Generate Visual Companion HTML
    Companion,
    /// Generate Master Print Bundle
    PrintBundle,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let project_root = cli.project_root.canonicalize()?;

    match cli.command {
        Commands::Build => {
            println!("Building assembled pages...");
            assembler::run(&project_root)?;
        }
        Commands::Validate => {
            validator::run(&project_root)?;
        }
        Commands::Companion => {
            println!("Generating Visual Companion...");
            companion::run(&project_root)?;
        }
        Commands::PrintBundle => {
            println!("Generating Print Bundle...");
            print_bundle::run(&project_root)?;
        }
    }
    Ok(())
}
