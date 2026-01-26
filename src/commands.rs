use semmap::{deps, formatter, generator, parser, validator, SemmapFile};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

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
            (Some(p), Some(l)) => format!("{p}:{l}"),
            (Some(p), None) => p.clone(),
            (None, Some(l)) => format!("line {l}"),
            (None, None) => String::new(),
        };

        if location.is_empty() {
            println!("{icon} {}", issue.message);
        } else {
            println!("{icon} [{location}] {}", issue.message);
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
        "✓ Generated {} ({} layers, {file_count} files)",
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
            println!("✓ No layer violations");
        } else {
            for v in &violations {
                println!("✗ {v}");
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

    let existing: HashSet<String> = semmap.all_paths().into_iter().map(String::from).collect();
    let current: HashSet<String> = fresh.all_paths().into_iter().map(String::from).collect();

    let added: Vec<String> = current.difference(&existing).cloned().collect();
    let removed: Vec<String> = existing.difference(&current).cloned().collect();

    let added_count = added.len();
    let removed_count = removed.len();

    let target_layer = guess_layer_for_new(&semmap);
    let layer_idx = semmap.layers.iter().position(|l| l.number == target_layer);

    for path in &added {
        if let Some(entry) = fresh.find_entry(path) {
            if let Some(idx) = layer_idx {
                if let Some(layer) = semmap.layers.get_mut(idx) {
                    layer.entries.push(entry.clone());
                }
            }
        }
    }

    let removed_set: HashSet<&str> = removed.iter().map(String::as_str).collect();
    for layer in &mut semmap.layers {
        layer
            .entries
            .retain(|e| !removed_set.contains(e.path.as_str()));
    }

    let output = formatter::to_markdown(&semmap);
    fs::write(file, &output).map_err(|e| format!("Failed to write {}: {e}", file.display()))?;

    println!("✓ Updated SEMMAP: +{added_count} -{removed_count}");
    for path in &added {
        println!("  + {path}");
    }
    for path in &removed {
        println!("  - {path}");
    }

    Ok(())
}

fn guess_layer_for_new(semmap: &SemmapFile) -> u8 {
    semmap.layers.first().map_or(2, |l| l.number)
}
