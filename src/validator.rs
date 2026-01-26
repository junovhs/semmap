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
        self.issues.iter().filter(|i| i.severity == Severity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == Severity::Warning).count()
    }
}

pub fn validate(semmap: &SemmapFile, root: Option<&Path>) -> ValidationResult {
    let mut issues = Vec::new();

    check_header(semmap, &mut issues);
    check_layers(semmap, &mut issues);
    check_entries(semmap, &mut issues);
    check_duplicates(semmap, &mut issues);
    
    if let Some(root_path) = root {
        check_files_exist(semmap, root_path, &mut issues);
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

    if semmap.purpose.len() > 200 {
        issues.push(ValidationIssue::warning(
            "Purpose should be one concise sentence (under 200 chars)"
        ));
    }
}

fn check_layers(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    if semmap.layers.is_empty() {
        issues.push(ValidationIssue::error("No layers defined"));
        return;
    }

    let mut seen_numbers: HashSet<u8> = HashSet::new();
    let mut prev_num: Option<u8> = None;

    for layer in &semmap.layers {
        if seen_numbers.contains(&layer.number) {
            issues.push(ValidationIssue::error(
                format!("Duplicate layer number: {}", layer.number)
            ));
        }
        seen_numbers.insert(layer.number);

        if let Some(prev) = prev_num {
            if layer.number != prev + 1 {
                issues.push(ValidationIssue::warning(
                    format!("Layer {} should follow {} (gap detected)", layer.number, prev)
                ));
            }
        }
        prev_num = Some(layer.number);

        if layer.name.is_empty() {
            issues.push(ValidationIssue::warning(
                format!("Layer {} has no name", layer.number)
            ));
        }

        if layer.entries.is_empty() {
            issues.push(ValidationIssue::warning(
                format!("Layer {} ({}) has no entries", layer.number, layer.name)
            ));
        }
    }
}

fn check_entries(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    for layer in &semmap.layers {
        for entry in &layer.entries {
            validate_entry(entry, issues);
        }
    }
}

fn validate_entry(entry: &crate::types::FileEntry, issues: &mut Vec<ValidationIssue>) {
    if entry.path.is_empty() {
        issues.push(ValidationIssue::error("Empty file path"));
        return;
    }

    if entry.description.what.is_empty() {
        issues.push(
            ValidationIssue::error("Missing WHAT description")
                .for_path(&entry.path)
        );
    }

    if entry.description.why.is_empty() {
        issues.push(
            ValidationIssue::warning("Missing WHY description")
                .for_path(&entry.path)
        );
    }

    if !entry.description.what.ends_with('.') && !entry.description.what.is_empty() {
        issues.push(
            ValidationIssue::warning("WHAT should end with a period")
                .for_path(&entry.path)
        );
    }
}

fn check_duplicates(semmap: &SemmapFile, issues: &mut Vec<ValidationIssue>) {
    let mut seen_paths: HashSet<&str> = HashSet::new();

    for layer in &semmap.layers {
        for entry in &layer.entries {
            if seen_paths.contains(entry.path.as_str()) {
                issues.push(
                    ValidationIssue::error("Duplicate path")
                        .for_path(&entry.path)
                );
            }
            seen_paths.insert(&entry.path);
        }
    }
}

fn check_files_exist(semmap: &SemmapFile, root: &Path, issues: &mut Vec<ValidationIssue>) {
    for layer in &semmap.layers {
        for entry in &layer.entries {
            let full_path = root.join(&entry.path);
            if !full_path.exists() {
                issues.push(
                    ValidationIssue::error("File not found")
                        .for_path(&entry.path)
                );
            }
        }
    }
}

pub fn validate_against_codebase(semmap: &SemmapFile, root: &Path) -> ValidationResult {
    let mut issues = validate(semmap, Some(root)).issues;
    
    let documented: HashSet<_> = semmap.all_paths().into_iter().collect();
    let actual = collect_source_files(root);

    for file in &actual {
        if !documented.contains(file.as_str()) {
            issues.push(
                ValidationIssue::warning("File not documented in SEMMAP")
                    .for_path(file)
            );
        }
    }

    ValidationResult { issues }
}

fn collect_source_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let walker = walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_ignored(e));

    for entry in walker.filter_map(Result::ok) {
        if entry.file_type().is_file() && is_source_file(entry.path()) {
            if let Ok(rel) = entry.path().strip_prefix(root) {
                files.push(rel.to_string_lossy().to_string());
            }
        }
    }

    files
}

fn is_ignored(entry: &walkdir::DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    name.starts_with('.') || name == "target" || name == "node_modules"
}

fn is_source_file(path: &Path) -> bool {
    let exts = ["rs", "ts", "js", "py", "go", "java", "c", "cpp", "h"];
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| exts.contains(&e))
}
