#![allow(clippy::unwrap_used, clippy::expect_used)]

use semmap::commands;
use std::fs;

#[test]
fn validate_fails_on_missing_file() {
    let tmp = tempfile::tempdir().unwrap();
    let semmap = tmp.path().join("test.md");
    fs::write(
        &semmap,
        "# Test -- Semantic Map\n\n## Layer 0\n`nonexistent.rs`\ndesc",
    )
    .unwrap();

    let result = commands::validate(&semmap, tmp.path(), false);
    assert!(result.is_err(), "Should fail when file doesn't exist");

    let err_msg = result.expect_err("should be error");
    assert!(err_msg.contains("not found") || err_msg.contains("error"));
}

#[test]
fn generate_json_format_arm_not_deleted() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

    let out = tmp.path().join("out.json");
    commands::generate(tmp.path(), &out, Some("testproj".into()), None, "json").unwrap();

    let content = fs::read_to_string(&out).unwrap();
    assert!(
        !content.starts_with('#'),
        "Output should be JSON, not Markdown"
    );
    assert!(
        content.contains("\"project_name\""),
        "Should contain JSON key"
    );
}

#[test]
fn generate_toml_format_arm_not_deleted() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

    let out = tmp.path().join("out.toml");
    commands::generate(tmp.path(), &out, Some("testproj".into()), None, "toml").unwrap();

    let content = fs::read_to_string(&out).unwrap();
    assert!(
        !content.starts_with("# testproj -- Semantic Map"),
        "Should be TOML"
    );
    assert!(
        content.contains("project_name = "),
        "Should contain TOML key-value"
    );
}
