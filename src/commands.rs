use crate::error::Severity;
use crate::{deps, formatter, generator, parser, path_utils, validator};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

mod update_helpers;
use update_helpers::{add_new_entries, remove_deleted_entries};

pub fn validate(file: &Path, root: &Path, strict: bool) -> Result<(), String> {
    let content =
        fs::read_to_string(file).map_err(|e| format!("Failed to read {}: {e}", file.display()))?;
    let semmap = parser::parse(&content).map_err(|e| format!("Parse error: {e}"))?;
    let result = if strict {
        validator::validate_against_codebase(&semmap, root)
    } else {
        validator::validate(&semmap, Some(root))
    };
    print_validation_result(&result);

    let has_errors = result.error_count() > 0;
    let has_warnings = result.warning_count() > 0;

    if has_errors || (strict && has_warnings) {
        Err(format!(
            "{} errors, {} warnings",
            result.error_count(),
            result.warning_count()
        ))
    } else {
        println!("* SEMMAP is valid");
        Ok(())
    }
}

fn print_validation_result(result: &validator::ValidationResult) {
    for issue in &result.issues {
        let icon = if issue.severity == Severity::Error {
            "X"
        } else {
            "!"
        };
        match (&issue.path, &issue.line) {
            (Some(p), Some(l)) => println!("{icon} [{p}:{l}] {}", issue.message),
            (Some(p), None) => println!("{icon} [{p}] {}", issue.message),
            (None, Some(l)) => println!("{icon} [line {l}] {}", issue.message),
            (None, None) => println!("{icon} {}", issue.message),
        }
    }
    if !result.issues.is_empty() {
        println!();
    }
}

pub fn generate(
    root: &Path,
    output: &Path,
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
        "json" => formatter::to_json(&semmap).map_err(|e| format!("JSON error: {e}"))?,
        "toml" => formatter::to_toml(&semmap).map_err(|e| format!("TOML error: {e}"))?,
        _ => formatter::to_markdown(&semmap),
    };
    fs::write(output, &content)
        .map_err(|e| format!("Failed to write {}: {e}", output.display()))?;
    let file_count: usize = semmap.layers.iter().map(|l| l.entries.len()).sum();
    println!(
        "* Generated {} ({} layers, {file_count} files)",
        output.display(),
        semmap.layers.len()
    );
    Ok(())
}

pub fn deps(file: &Path, root: &Path, format: &str, check: bool) -> Result<(), String> {
    let content =
        fs::read_to_string(file).map_err(|e| format!("Failed to read {}: {e}", file.display()))?;
    let semmap = parser::parse(&content).map_err(|e| format!("Parse error: {e}"))?;
    let depmap = deps::analyze(root, &semmap);
    if check {
        let violations = deps::check_layer_violations(&depmap, &semmap);
        if violations.is_empty() {
            println!("* No layer violations");
        } else {
            for v in &violations {
                println!("X {v}");
            }
            return Err(format!("{} layer violations", violations.len()));
        }
    }
    let output = match format {
        "json" => serde_json::to_string_pretty(&depmap).map_err(|e| format!("JSON error: {e}"))?,
        _ => deps::render_mermaid(&depmap),
    };
    println!("{output}");
    Ok(())
}

pub fn update(file: &Path, root: &Path) -> Result<(), String> {
    let content =
        fs::read_to_string(file).map_err(|e| format!("Failed to read {}: {e}", file.display()))?;
    let mut semmap = parser::parse(&content).map_err(|e| format!("Parse error: {e}"))?;
    let fresh = generator::generate(
        root,
        generator::GeneratorConfig {
            project_name: semmap.project_name.clone(),
            purpose: semmap.purpose.clone(),
            ..Default::default()
        },
    );
    let semmap_dir = file.parent().unwrap_or(Path::new("."));
    let root_prefix = path_utils::build_root_prefix_relative(semmap_dir, root);
    let existing: HashSet<String> = semmap.all_paths().into_iter().map(String::from).collect();
    let current: HashSet<String> = fresh
        .all_paths()
        .into_iter()
        .map(|p| path_utils::prefix_path(&root_prefix, p))
        .collect();
    let added: Vec<String> = current.difference(&existing).cloned().collect();
    let removed: Vec<String> = existing.difference(&current).cloned().collect();

    add_new_entries(&mut semmap, &added, &fresh, &root_prefix);
    remove_deleted_entries(&mut semmap, &removed);

    let output = formatter::to_markdown(&semmap);
    fs::write(file, &output).map_err(|e| format!("Failed to write {}: {e}", file.display()))?;
    println!("* Updated SEMMAP: +{} -{}", added.len(), removed.len());
    for path in &added {
        println!("  + {path}");
    }
    for path in &removed {
        println!("  - {path}");
    }
    Ok(())
}
