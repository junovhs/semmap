//! Validates SEMMAP files for correctness and completeness.

use crate::error::{Severity, ValidationIssue};
use crate::types::SemmapFile;
use std::collections::HashSet;
use std::path::Path;

pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        !self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count()
    }
}

pub fn validate(semmap: &SemmapFile, root: Option<&Path>) -> ValidationResult {
    let mut issues = Vec::new();
    check_header(semmap, &mut issues);
    check_layers(semmap, &mut issues);
    check_entries(semmap, &mut issues);
    check_duplicates(semmap, &mut issues);
    if let Some(r) = root {
        check_files_exist(semmap, r, &mut issues);
    }
    ValidationResult { issues }
}

fn check_header(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    if semmap.project_name.is_empty() {
        issues.push(ValidationIssue::error("Missing project name"));
    }
    if semmap.purpose.is_empty() {
        issues.push(ValidationIssue::warning("Missing purpose statement"));
    }
}

fn check_layers(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    if semmap.layers.is_empty() {
        issues.push(ValidationIssue::error("No layers defined"));
        return;
    }
    let mut seen: HashSet<u8> = HashSet::new();
    let mut prev: Option<u8> = None;
    for layer in &semmap.layers {
        if seen.contains(&layer.number) {
            issues.push(ValidationIssue::error(format!(
                "Duplicate layer: {}",
                layer.number
            )));
        }
        seen.insert(layer.number);
        if let Some(p) = prev {
            if layer.number != p + 1 {
                issues.push(ValidationIssue::warning(format!("Layer gap after {p}")));
            }
        }
        prev = Some(layer.number);
    }
}

fn check_entries(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    for layer in &semmap.layers {
        for entry in &layer.entries {
            if entry.description.what.is_empty() {
                issues.push(ValidationIssue::error("Missing WHAT").for_path(&entry.path));
            }
            if is_generic_description(&entry.description.what, &entry.path) {
                issues.push(ValidationIssue::warning("Add //! doc comment").for_path(&entry.path));
            }
        }
    }
}

fn is_generic_description(what: &str, path: &str) -> bool {
    let p = Path::new(path);
    let is_config = p
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_lowercase().as_str(), "toml" | "json" | "yaml"));
    if is_config {
        return false;
    }
    what.starts_with("Implements ") || what.contains("functionality.")
}

fn check_duplicates(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    let mut seen: HashSet<&str> = HashSet::new();
    for layer in &semmap.layers {
        for entry in &layer.entries {
            if seen.contains(entry.path.as_str()) {
                issues.push(ValidationIssue::error("Duplicate path").for_path(&entry.path));
            }
            seen.insert(&entry.path);
        }
    }
}

fn check_files_exist(semmap: &SemmapFile, root: &Path, issues: &mut Vec<ValidationIssue>) {
    for layer in &semmap.layers {
        for entry in &layer.entries {
            if !root.join(&entry.path).exists() {
                issues.push(ValidationIssue::error("File not found").for_path(&entry.path));
            }
        }
    }
}

pub fn validate_against_codebase(semmap: &SemmapFile, root: &Path) -> ValidationResult {
    let mut issues = validate(semmap, Some(root)).issues;
    let documented: HashSet<_> = semmap.all_paths().into_iter().collect();

    for file in collect_source_files(root) {
        if !documented.contains(file.as_str()) {
            issues.push(ValidationIssue::warning("Not in SEMMAP").for_path(&file));
        }
    }
    ValidationResult { issues }
}

fn collect_source_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let walker = walkdir::WalkDir::new(root).into_iter().filter_entry(|e| {
        let n = e.file_name().to_string_lossy();
        !n.starts_with('.') && n != "target" && n != "node_modules"
    });

    for entry in walker.filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let is_src = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| matches!(e, "rs" | "ts" | "js" | "py" | "go"));
            if is_src {
                if let Ok(rel) = entry.path().strip_prefix(root) {
                    files.push(rel.to_string_lossy().replace('\\', "/"));
                }
            }
        }
    }
    files
}
