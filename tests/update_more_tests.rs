use semmap::commands;
use semmap::parser;
use std::fs;
use tempfile::TempDir;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn create_semmap(root: &std::path::Path, content: &str) -> TestResult {
    fs::write(root.join("SEMMAP.md"), content)?;
    Ok(())
}

fn parse_semmap(root: &std::path::Path) -> Result<semmap::SemmapFile, String> {
    let content = fs::read_to_string(root.join("SEMMAP.md")).map_err(|e| e.to_string())?;
    parser::parse(&content).map_err(|e| format!("{e}"))
}

#[test]
fn test_update_is_idempotent() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_semmap(root, "# Test -- Semantic Map\nPurpose: Test.\n\n## Layer 0 -- Config\n`Cargo.toml`\nManifest. Build.\n")?;
    fs::write(root.join("Cargo.toml"), "")?;
    fs::write(root.join("lib.rs"), "")?;

    commands::update(&root.join("SEMMAP.md"), root)?;
    let first_output = fs::read_to_string(root.join("SEMMAP.md"))?;

    commands::update(&root.join("SEMMAP.md"), root)?;
    let second_output = fs::read_to_string(root.join("SEMMAP.md"))?;

    assert_eq!(first_output, second_output,
        "Update should be idempotent");

    let semmap = parse_semmap(root)?;
    let layer1 = semmap.layers.iter().find(|l| l.number == 1)
        .ok_or("Layer 1 should exist")?;
    let lib_count = layer1.entries.iter().filter(|e| e.path == "lib.rs").count();
    assert_eq!(lib_count, 1, "lib.rs should appear exactly once");

    Ok(())
}

#[test]
fn test_update_preserves_existing_descriptions() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    let custom_desc = "Custom WHAT. Custom WHY.";
    create_semmap(root, &format!("# Test -- Semantic Map\nPurpose: Test.\n\n## Layer 2 -- Domain\n`keep.rs`\n{custom_desc}\n"))?;
    fs::write(root.join("keep.rs"), "")?;
    fs::write(root.join("new.rs"), "")?;

    commands::update(&root.join("SEMMAP.md"), root)?;
    let semmap = parse_semmap(root)?;
    let keep_entry = semmap.find_entry("keep.rs").ok_or("keep.rs missing")?;

    assert_eq!(keep_entry.description.what, "Custom WHAT.",
        "Existing descriptions should be preserved");
    assert_eq!(keep_entry.description.why, "Custom WHY.",
        "Existing WHY should be preserved");

    let new_entry = semmap.find_entry("new.rs").ok_or("new.rs missing")?;
    assert!(!new_entry.description.what.is_empty(),
        "New file should have generated description");

    Ok(())
}

#[test]
fn test_update_creates_missing_layers_for_new_entries() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_semmap(root, "# Test -- Semantic Map\nPurpose: Test.\n\n## Layer 0 -- Config\n`Cargo.toml`\nManifest. Build.\n")?;
    fs::write(root.join("Cargo.toml"), "")?;
    fs::write(root.join("main.rs"), "fn main() {}")?;
    fs::write(root.join("utils.rs"), "")?;
    fs::write(root.join("test.rs"), "#[test] fn t() {}")?;

    commands::update(&root.join("SEMMAP.md"), root)?;
    let semmap = parse_semmap(root)?;

    let layer_numbers: Vec<_> = semmap.layers.iter().map(|l| l.number).collect();
    assert!(layer_numbers.contains(&0));
    assert!(layer_numbers.contains(&1), "Layer 1 should be created for main.rs");
    assert!(layer_numbers.contains(&3), "Layer 3 should be created for utils.rs");
    assert!(layer_numbers.contains(&4), "Layer 4 should be created for test.rs");

    let mut sorted = layer_numbers.clone();
    sorted.sort_unstable();
    assert_eq!(layer_numbers, sorted, "Layers should be sorted by number");

    let l1 = semmap.layers.iter().find(|l| l.number == 1)
        .ok_or("Layer 1 should exist for main.rs")?;
    assert!(l1.entries.iter().any(|e| e.path == "main.rs"));

    Ok(())
}
