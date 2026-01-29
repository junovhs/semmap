use semmap::commands;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "semmap")]
#[command(about = "Semantic Map generator and validator for codebases")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a SEMMAP file
    Validate {
        #[arg(short, long, default_value = "SEMMAP.md")]
        file: PathBuf,
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        #[arg(long)]
        strict: bool,
    },
    /// Generate a new SEMMAP from a codebase
    Generate {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        #[arg(short, long, default_value = "SEMMAP.md")]
        output: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        purpose: Option<String>,
        #[arg(long, default_value = "md")]
        format: String,
    },
    /// Analyze dependencies and generate a dependency map
    Deps {
        #[arg(short, long, default_value = "SEMMAP.md")]
        file: PathBuf,
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        #[arg(long, default_value = "mermaid")]
        format: String,
        #[arg(long)]
        check: bool,
    },
    /// Update an existing SEMMAP with new/removed files
    Update {
        #[arg(short, long, default_value = "SEMMAP.md")]
        file: PathBuf,
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Validate { file, root, strict } => commands::validate(&file, &root, strict),
        Commands::Generate { root, output, name, purpose, format } => {
            commands::generate(&root, &output, name, purpose, &format)
        }
        Commands::Deps { file, root, format, check } => {
            commands::deps(&file, &root, &format, check)
        }
        Commands::Update { file, root } => commands::update(&file, &root),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
