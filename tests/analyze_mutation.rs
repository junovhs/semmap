#![allow(clippy::unwrap_used)]

use semmap::deps;
use semmap::types::{DepKind, FileEntry, Layer};
use semmap::SemmapFile;
use std::fs;

#[test]
fn analyze_finds_rust_imports() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    fs::create_dir(root.join("src")).unwrap();
    fs::write(root.join("src/main.rs"), "use crate::utils;\nfn main() {}").unwrap();
    fs::write(root.join("src/utils.rs"), "").unwrap();

    let mut layer = Layer::new(1, "Core".into());
    layer.entries.push(FileEntry::new(
        "src/main.rs".into(),
        "desc".into(),
        String::new(),
    ));
    layer.entries.push(FileEntry::new(
        "src/utils.rs".into(),
        "desc".into(),
        String::new(),
    ));

    let semmap = SemmapFile {
        project_name: "test".into(),
        purpose: String::new(),
        legend: vec![],
        layers: vec![layer],
    };

    let depmap = deps::analyze(root, &semmap);

    assert!(!depmap.edges.is_empty(), "Should find imports");
    assert!(depmap
        .edges
        .iter()
        .any(|e| e.from == "src/main.rs" && e.to == "src/utils.rs" && e.kind == DepKind::Import));
}
