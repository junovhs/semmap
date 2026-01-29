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
fn test_update_adds_new_file_to_correct_layer() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_semmap(root, "# Test -- Semantic Map\nPurpose: Test.\n\n## Layer 0 -- Config\n`Cargo.toml`\nManifest. Build config.\n")?;
    fs::write(root.join("Cargo.toml"), "")?;
    fs::write(root.join("logic.rs"), "pub fn compute() {}")?;

    commands::update(&root.join("SEMMAP.md"), root)?;
    let semmap = parse_semmap(root)?;

    let layer2 = semmap.layers.iter().find(|l| l.number == 2)
        .ok_or("Layer 2 should exist for new .rs file")?;
    assert!(layer2.entries.iter().any(|e| e.path == "logic.rs"),
        "New Rust file should be in Layer 2 (Domain), not Layer 0");

    let layer0 = semmap.layers.iter().find(|l| l.number == 0)
        .ok_or("Layer 0 missing")?;
    assert!(!layer0.entries.iter().any(|e| e.path == "logic.rs"),
        "logic.rs should NOT be in Layer 0");

    Ok(())
}

#[test]
fn test_update_with_root_prefix_handles_paths_correctly() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path().join("crates").join("app");
    fs::create_dir_all(&root)?;

    let semmap_content = "# App -- Semantic Map\nPurpose: App.\n\n## Layer 0 -- Config\n`crates/app/Cargo.toml`\nManifest. Build.\n";
    fs::write(temp.path().join("SEMMAP.md"), semmap_content)?;
    fs::write(root.join("Cargo.toml"), "")?;
    fs::write(root.join("main.rs"), "fn main() {}")?;

    commands::update(&temp.path().join("SEMMAP.md"), &root)?;
    let semmap = parse_semmap(temp.path())?;

    let all_paths = semmap.all_paths();
    assert!(all_paths.contains(&"crates/app/main.rs"),
        "New file should have full prefixed path");
    assert!(all_paths.contains(&"crates/app/Cargo.toml"),
        "Existing prefixed path should be preserved");
    assert!(!all_paths.contains(&"main.rs"),
        "Paths should maintain root prefix");

    Ok(())
}

#[test]
fn test_update_removes_deleted_files() -> TestResult {
    let temp = TempDir::new()?;
    let root = temp.path();

    create_semmap(root, "# Test -- Semantic Map\nPurpose: Test.\n\n## Layer 0 -- Config\n`exists.toml`\nExists. Yes.\n\n## Layer 2 -- Domain\n`deleted.rs`\nGone. Deleted.\n")?;
    fs::write(root.join("exists.toml"), "")?;

    commands::update(&root.join("SEMMAP.md"), root)?;
    let semmap = parse_semmap(root)?;

    assert!(semmap.find_entry("exists.toml").is_some(),
        "Existing file should remain");
    assert!(semmap.find_entry("deleted.rs").is_none(),
        "Deleted file should be removed");

    let layer2_exists = semmap.layers.iter().any(|l| l.number == 2);
    assert!(!layer2_exists, "Layer 2 should be removed when empty");

    Ok(())
}
