use semmap::error::Severity;
use semmap::types::{Description, FileEntry, Layer, SemmapFile};
use semmap::validator::validate;
use std::error::Error;
use tempfile::TempDir;

type TestResult = Result<(), Box<dyn Error>>;

fn entry(path: &str, what: &str, why: &str) -> FileEntry {
    FileEntry {
        path: path.to_string(),
        description: Description {
            what: what.to_string(),
            why: why.to_string(),
        },
        exports: None,
        touch: None,
    }
}

fn layer(number: u8, name: &str, entries: Vec<FileEntry>) -> Layer {
    Layer {
        number,
        name: name.to_string(),
        entries,
    }
}

fn semmap(name: &str, purpose: &str, layers: Vec<Layer>) -> SemmapFile {
    SemmapFile {
        project_name: name.to_string(),
        purpose: purpose.to_string(),
        legend: vec![],
        layers,
    }
}

fn has_issue(result: &semmap::validator::ValidationResult, sev: Severity, msg: &str) -> bool {
    result
        .issues
        .iter()
        .any(|i| i.severity == sev && i.message.contains(msg))
}

/// A valid SEMMAP with proper structure should pass validation.
#[test]
fn test_valid_semmap_passes() {
    let s = semmap(
        "test-project",
        "A test project.",
        vec![layer(
            0,
            "Config",
            vec![entry("Cargo.toml", "Build config.", "Required.")],
        )],
    );
    let result = validate(&s, None);
    assert!(result.is_valid(), "Valid SEMMAP should pass");
    assert_eq!(result.error_count(), 0);
}

/// Missing project name should produce an error.
#[test]
fn test_missing_project_name_is_error() {
    let s = semmap(
        "",
        "Some purpose.",
        vec![layer(0, "Config", vec![entry("f.rs", "What.", "Why.")])],
    );
    let result = validate(&s, None);
    assert!(!result.is_valid(), "Missing project name should fail");
    assert!(has_issue(&result, Severity::Error, "project name"));
}

/// Missing purpose should produce a warning, not an error.
#[test]
fn test_missing_purpose_is_warning() {
    let s = semmap(
        "my-project",
        "",
        vec![layer(0, "Config", vec![entry("f.rs", "What.", "Why.")])],
    );
    let result = validate(&s, None);
    assert!(result.is_valid(), "Missing purpose should still be valid");
    assert!(has_issue(&result, Severity::Warning, "purpose"));
}

/// Duplicate paths across layers should produce an error.
#[test]
fn test_duplicate_paths_is_error() {
    let s = semmap(
        "test",
        "Test.",
        vec![
            layer(0, "Config", vec![entry("dup.rs", "First.", "Why.")]),
            layer(1, "Core", vec![entry("dup.rs", "Duplicate.", "Why.")]),
        ],
    );
    let result = validate(&s, None);
    assert!(!result.is_valid(), "Duplicate paths should fail");
    assert!(has_issue(&result, Severity::Error, "Duplicate"));
}

/// File not found on disk should produce an error when root is provided.
#[test]
fn test_file_not_found_is_error() -> TestResult {
    let temp = TempDir::new()?;
    let s = semmap(
        "test",
        "Test.",
        vec![layer(
            0,
            "Config",
            vec![entry("nonexistent.rs", "Missing.", "Should error.")],
        )],
    );
    let result = validate(&s, Some(temp.path()));
    assert!(!result.is_valid(), "Missing file should fail");
    assert!(has_issue(&result, Severity::Error, "not found"));
    Ok(())
}

/// Layer gap (e.g., 0 then 2) should produce a warning.
#[test]
fn test_layer_gap_is_warning() {
    let s = semmap(
        "test",
        "Test.",
        vec![
            layer(0, "Config", vec![entry("a.rs", "A.", "Why.")]),
            layer(2, "Skipped", vec![entry("b.rs", "B.", "Why.")]),
        ],
    );
    let result = validate(&s, None);
    assert!(result.is_valid(), "Layer gap should be warning, not error");
    assert!(has_issue(&result, Severity::Warning, "gap"));
}

/// No layers defined should produce an error.
#[test]
fn test_no_layers_is_error() {
    let s = semmap("test", "Test.", vec![]);
    let result = validate(&s, None);
    assert!(!result.is_valid(), "No layers should fail");
    assert!(has_issue(&result, Severity::Error, "No layers"));
}

/// Missing WHAT description should produce an error.
#[test]
fn test_missing_what_is_error() {
    let s = semmap(
        "test",
        "Test.",
        vec![layer(0, "Config", vec![entry("f.rs", "", "Why.")])],
    );
    let result = validate(&s, None);
    assert!(!result.is_valid(), "Missing WHAT should fail");
}
