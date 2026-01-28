use semmap::generator::{self, GeneratorConfig};
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn create_file(dir: &std::path::Path, name: &str, content: &str) -> TestResult {
    let path = dir.join(name);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?; // Ensure flush
    Ok(())
}

/// **Goal**: Debugging WalkDir behavior in the test environment.
#[test]
fn debug_walkdir() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    create_file(root, "Cargo.toml", "[package]").unwrap();

    let walker = walkdir::WalkDir::new(root).follow_links(false);
    let entries: Vec<_> = walker.into_iter().filter_map(|e| e.ok()).collect();

    println!("DEBUG: Root path: {:?}", root);
    println!("DEBUG: WalkDir found {} entries", entries.len());
    for entry in &entries {
        println!("DEBUG: Entry: {:?}", entry.path());
        let name = entry.file_name().to_string_lossy();
        let is_excluded = name.starts_with('.');
        println!("DEBUG:   is_excluded by dot? {}", is_excluded);
    }
}

/// **Goal**: Verify that the generator detects a `Cargo.toml` file and correctly assigns it to Layer 0 (Config).
/// **Justification**: This is a critical heuristic. If config files aren't identified, the entire layer structure is invalid.
#[test]
fn test_generate_finds_config_layer0() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_file(root, "Cargo.toml", "[package]")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer0 = semmap.layers.iter().find(|l| l.number == 0).ok_or("Layer 0 missing")?;
    let entry = layer0.entries.iter().find(|e| e.path == "Cargo.toml").ok_or("Cargo.toml missing")?;
    
    assert_eq!(entry.description.what, "Rust package manifest and dependencies.");
    Ok(())
}

/// **Goal**: Verify that general Rust files are assigned to Layer 2 (Domain/Core) by default if they don't match specific heuristics.
/// **Justification**: This ensures that we have a safe fallback for unknown files, preventing them from being lost or misclassified as config/test.
#[test]
fn test_generate_defaults_to_layer2() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_file(root, "my_logic.rs", "fn logic() {}")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    // Default layer is 2
    let layer2 = semmap.layers.iter().find(|l| l.number == 2).ok_or("Layer 2 missing")?;
    let entry = layer2.entries.iter().find(|e| e.path == "my_logic.rs").ok_or("my_logic.rs missing")?;
    
    assert!(entry.description.what.contains("Implements my_logic functionality"));
    Ok(())
}

/// **Goal**: Verify that files with "test" in the name are correctly assigned to Layer 4 (Tests).
/// **Justification**: Separation of tests from production code is essential for a semantic map. Misclassifying tests as core logic creates noise.
#[test]
fn test_generate_identifies_tests_layer4() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    let src = root.join("src");
    fs::create_dir(&src)?;
    create_file(&src, "my_test.rs", "#[test] fn t() {}")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer4 = semmap.layers.iter().find(|l| l.number == 4).ok_or("Layer 4 missing")?;
    
    // Normalize path check
    let found = layer4.entries.iter().any(|e| e.path.replace('\\', "/") == "src/my_test.rs");
    assert!(found, "src/my_test.rs not found in layer 4");
    Ok(())
}

/// **Goal**: Verify that public items (structs, functions, enums, traits) are correctly extracted into the `exports` field.
/// **Justification**: The `Exports` field provides a high-level API summary. We need to ensure the parser correctly identifies `pub` keywords and extracts names.
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

    // 2 is default layer
    let entry = semmap.find_entry("auth.rs").ok_or("auth.rs missing")?;
    
    let exports = entry.exports.as_ref().ok_or("Exports missing")?;
    assert!(exports.contains(&"User".to_string()));
    assert!(exports.contains(&"login".to_string()));
    assert!(exports.contains(&"Role".to_string()));
    assert!(exports.contains(&"Auth".to_string()));
    assert!(!exports.contains(&"Private".to_string()));
    Ok(())
}

/// **Goal**: Verify that files in excluded directories (e.g., `node_modules`, target) are ignored.
/// **Justification**: Scanning build artifacts or dependencies is a huge performance waste and creates a massive, useless map.
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

/// **Goal**: Verify that inference.rs correctly deduces `main` and `lib` as Layer 1 (Core).
/// **Justification**: Entry points are the most important part of the application structure. They must be highlighted in Layer 1.
#[test]
fn test_inference_main_lib_layer1() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_file(root, "main.rs", "fn main() {}")?;
    create_file(root, "lib.rs", "")?;

    let config = GeneratorConfig::default();
    let semmap = generator::generate(root, config);

    let layer1 = semmap.layers.iter().find(|l| l.number == 1).ok_or("Layer 1 missing")?;
    
    assert!(layer1.entries.iter().any(|e| e.path == "main.rs"));
    assert!(layer1.entries.iter().any(|e| e.path == "lib.rs"));
    Ok(())
}
