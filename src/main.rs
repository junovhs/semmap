use clap::{Parser, Subcommand};
use semmap::{deps, formatter, generator, parser, validator, SemmapFile};
use std::fs;
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
        /// Path to the SEMMAP markdown file
        #[arg(short, long, default_value = "SEMMAP.md")]
        file: PathBuf,

        /// Root directory to check file existence
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Compare against actual codebase
        #[arg(long)]
        strict: bool,
    },

    /// Generate a new SEMMAP from a codebase
    Generate {
        /// Root directory of the codebase
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Output file path
        #[arg(short, long, default_value = "SEMMAP.md")]
        output: PathBuf,

        /// Project name (defaults to directory name)
        #[arg(long)]
        name: Option<String>,

        /// Project purpose statement
        #[arg(long)]
        purpose: Option<String>,

        /// Output format: md, json, toml
        #[arg(long, default_value = "md")]
        format: String,
    },

    /// Analyze dependencies and generate a dependency map
    Deps {
        /// Path to the SEMMAP file
        #[arg(short, long, default_value = "SEMMAP.md")]
        file: PathBuf,

        /// Root directory of codebase
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Output format: mermaid, json
        #[arg(long, default_value = "mermaid")]
        format: String,

        /// Check for layer violations
        #[arg(long)]
        check: bool,
    },

    /// Update an existing SEMMAP with new/removed files
    Update {
        /// Path to the SEMMAP file
        #[arg(short, long, default_value = "SEMMAP.md")]
        file: PathBuf,

        /// Root directory of codebase
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Validate { file, root, strict } => cmd_validate(&file, &root, strict),
        Commands::Generate { root, output, name, purpose, format } => {
            cmd_generate(&root, &output, name, purpose, &format)
        }
        Commands::Deps { file, root, format, check } => {
            cmd_deps(&file, &root, &format, check)
        }
        Commands::Update { file, root } => cmd_update(&file, &root),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_validate(file: &PathBuf, root: &PathBuf, strict: bool) -> Result<(), String> {
    let content = fs::read_to_string(file)
        .map_err(|e| format!("Failed to read {}: {}", file.display(), e))?;

    let semmap = parser::parse(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    let result = if strict {
        validator::validate_against_codebase(&semmap, root)
    } else {
        validator::validate(&semmap, Some(root))
    };

    print_validation_result(&result);

    if result.is_valid() {
        println!("✓ SEMMAP is valid");
        Ok(())
    } else {
        Err(format!("{} errors found", result.error_count()))
    }
}

fn print_validation_result(result: &validator::ValidationResult) {
    for issue in &result.issues {
        let icon = match issue.severity {
            semmap::error::Severity::Error => "✗",
            semmap::error::Severity::Warning => "⚠",
        };

        let location = match (&issue.path, &issue.line) {
            (Some(p), Some(l)) => format!("{}:{}", p, l),
            (Some(p), None) => p.clone(),
            (None, Some(l)) => format!("line {}", l),
            (None, None) => String::new(),
        };

        if location.is_empty() {
            println!("{} {}", icon, issue.message);
        } else {
            println!("{} [{}] {}", icon, location, issue.message);
        }
    }

    if !result.issues.is_empty() {
        println!();
    }
}

fn cmd_generate(
    root: &PathBuf,
    output: &PathBuf,
    name: Option<String>,
    purpose: Option<String>,
    format: &str,
) -> Result<(), String> {
    let config = generator::GeneratorConfig {
        project_name: name.unwrap_or_default(),
        purpose: purpose.unwrap_or_default(),
        ..Default::default()
    };

    let semmap = generator::generate(root, config);

    let content = match format {
        "json" => formatter::to_json(&semmap)
            .map_err(|e| format!("JSON error: {}", e))?,
        "toml" => formatter::to_toml(&semmap)
            .map_err(|e| format!("TOML error: {}", e))?,
        _ => formatter::to_markdown(&semmap),
    };

    fs::write(output, &content)
        .map_err(|e| format!("Failed to write {}: {}", output.display(), e))?;

    println!("✓ Generated {} ({} layers, {} files)",
        output.display(),
        semmap.layers.len(),
        semmap.layers.iter().map(|l| l.entries.len()).sum::<usize>()
    );

    Ok(())
}

fn cmd_deps(file: &PathBuf, root: &PathBuf, format: &str, check: bool) -> Result<(), String> {
    let content = fs::read_to_string(file)
        .map_err(|e| format!("Failed to read {}: {}", file.display(), e))?;

    let semmap = parser::parse(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    let depmap = deps::analyze(root, &semmap);

    if check {
        let violations = deps::check_layer_violations(&depmap, &semmap);
        if violations.is_empty() {
            println!("✓ No layer violations");
        } else {
            for v in &violations {
                println!("✗ {}", v);
            }
            return Err(format!("{} layer violations", violations.len()));
        }
    }

    let output = match format {
        "json" => serde_json::to_string_pretty(&depmap)
            .map_err(|e| format!("JSON error: {}", e))?,
        _ => deps::render_mermaid(&depmap),
    };

    println!("{}", output);
    Ok(())
}

fn cmd_update(file: &PathBuf, root: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(file)
        .map_err(|e| format!("Failed to read {}: {}", file.display(), e))?;

    let mut semmap = parser::parse(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    let fresh = generator::generate(root, generator::GeneratorConfig {
        project_name: semmap.project_name.clone(),
        purpose: semmap.purpose.clone(),
        ..Default::default()
    });

    // Clone to owned strings to avoid borrow conflicts
    let existing: std::collections::HashSet<String> = semmap.all_paths()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let current: std::collections::HashSet<String> = fresh.all_paths()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let added: Vec<String> = current.difference(&existing).cloned().collect();
    let removed: Vec<String> = existing.difference(&current).cloned().collect();

    let added_count = added.len();
    let removed_count = removed.len();

    for path in &added {
        if let Some(entry) = fresh.find_entry(path) {
            let layer = guess_layer_for_new(&semmap, path);
            if let Some(l) = semmap.layers.iter_mut().find(|l| l.number == layer) {
                l.entries.push(entry.clone());
            }
        }
    }

    let removed_set: std::collections::HashSet<&str> = removed.iter().map(|s| s.as_str()).collect();
    for layer in &mut semmap.layers {
        layer.entries.retain(|e| !removed_set.contains(e.path.as_str()));
    }

    let output = formatter::to_markdown(&semmap);
    fs::write(file, &output)
        .map_err(|e| format!("Failed to write {}: {}", file.display(), e))?;

    println!("✓ Updated SEMMAP: +{} -{}", added_count, removed_count);
    for path in &added { println!("  + {}", path); }
    for path in &removed { println!("  - {}", path); }

    Ok(())
}

fn guess_layer_for_new(semmap: &SemmapFile, _path: &str) -> u8 {
    semmap.layers.first().map(|l| l.number).unwrap_or(2)
}
