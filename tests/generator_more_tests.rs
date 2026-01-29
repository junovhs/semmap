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

/// Verify that public items (structs, functions, enums, traits) are correctly extracted into the exports field.
/// The Exports field provides a high-level API summary.
#[test]
fn test_generate_extracts_exports() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    let code = r"
pub struct User { name: String }
struct Private;
pub fn login() {}
pub enum Role { Admin }
pub trait Auth {}
";
    create_file(root, "auth.rs", code)?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let entry = semmap.find_entry("auth.rs").ok_or("auth.rs missing")?;

    let exports = entry.exports.as_ref().ok_or("Exports missing")?;
    assert!(exports.contains(&"User".to_string()));
    assert!(exports.contains(&"login".to_string()));
    assert!(exports.contains(&"Role".to_string()));
    assert!(exports.contains(&"Auth".to_string()));
    assert!(!exports.contains(&"Private".to_string()));
    Ok(())
}

/// Verify that files in excluded directories (e.g., `node_modules`, target) are ignored.
/// Scanning build artifacts is wasteful and creates a bloated map.
#[test]
fn test_generate_respects_excludes() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    fs::create_dir(root.join("target"))?;
    create_file(root, "target/lib.rs", "")?;
    create_file(root, "keep_me.rs", "")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    assert!(semmap.find_entry("target/lib.rs").is_none());
    assert!(semmap.find_entry("keep_me.rs").is_some());
    Ok(())
}

/// Verify that inference.rs correctly deduces main and lib as Layer 1 (Core).
/// Entry points are the most important part of the application structure.
#[test]
fn test_inference_main_lib_layer1() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_file(root, "main.rs", "fn main() {}")?;
    create_file(root, "lib.rs", "")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer1 = semmap
        .layers
        .iter()
        .find(|l| l.number == 1)
        .ok_or("Layer 1 missing")?;

    assert!(layer1.entries.iter().any(|e| e.path == "main.rs"));
    assert!(layer1.entries.iter().any(|e| e.path == "lib.rs"));
    Ok(())
}
