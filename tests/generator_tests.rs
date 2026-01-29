use semmap::generator::{self, GeneratorConfig};
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn create_file(dir: &std::path::Path, name: &str, content: &str) -> TestResult {
    let path = dir.join(name);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// Verify that the generator detects a Cargo.toml file and correctly assigns it to Layer 0 (Config).
/// This is a critical heuristic. If config files aren't identified, the entire layer structure is invalid.
#[test]
fn test_generate_finds_config_layer0() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_file(root, "Cargo.toml", "[package]")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer0 = semmap
        .layers
        .iter()
        .find(|l| l.number == 0)
        .ok_or("Layer 0 missing")?;
    let entry = layer0
        .entries
        .iter()
        .find(|e| e.path == "Cargo.toml")
        .ok_or("Cargo.toml missing")?;

    assert_eq!(
        entry.description.what,
        "Rust package manifest and dependencies."
    );
    Ok(())
}

/// Verify that general Rust files are assigned to Layer 2 (Domain/Core) by default.
/// This ensures a safe fallback for unknown files, preventing misclassification.
#[test]
fn test_generate_defaults_to_layer2() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_file(root, "my_logic.rs", "fn logic() {}")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer2 = semmap
        .layers
        .iter()
        .find(|l| l.number == 2)
        .ok_or("Layer 2 missing")?;
    let entry = layer2
        .entries
        .iter()
        .find(|e| e.path == "my_logic.rs")
        .ok_or("my_logic.rs missing")?;

    // SWUM expands "my_logic" to "Implements my logic."
    assert!(
        entry.description.what.contains("my") && entry.description.what.contains("logic"),
        "Expected description to contain 'my' and 'logic', got: {}",
        entry.description.what
    );
    Ok(())
}

/// Verify that files with "test" in the name are correctly assigned to Layer 4 (Tests).
/// Separation of tests from production code is essential for a clean semantic map.
#[test]
fn test_generate_identifies_tests_layer4() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    let src = root.join("src");
    fs::create_dir(&src)?;
    create_file(&src, "my_test.rs", "#[test] fn t() {}")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer4 = semmap
        .layers
        .iter()
        .find(|l| l.number == 4)
        .ok_or("Layer 4 missing")?;

    let found = layer4
        .entries
        .iter()
        .any(|e| e.path.replace('\\', "/") == "src/my_test.rs");
    assert!(found, "src/my_test.rs not found in layer 4");
    Ok(())
}
