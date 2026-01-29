#![allow(clippy::unwrap_used)]

use semmap::deps;
use semmap::types::{DepEdge, DepKind, DepNode, DependencyMap, FileEntry, Layer};
use semmap::SemmapFile;

#[test]
fn layer_violation_detects_specific_patterns() {
    // Test case 1: No violation (same layer)
    let depmap_same = DependencyMap {
        nodes: vec![
            DepNode {
                path: "a.rs".into(),
                layer: 2,
            },
            DepNode {
                path: "b.rs".into(),
                layer: 2,
            },
        ],
        edges: vec![DepEdge {
            from: "a.rs".into(),
            to: "b.rs".into(),
            kind: DepKind::Import,
        }],
    };

    let mut layer2_same = Layer::new(2, "Domain".into());
    layer2_same
        .entries
        .push(FileEntry::new("a.rs".into(), "desc".into(), String::new()));
    layer2_same
        .entries
        .push(FileEntry::new("b.rs".into(), "desc".into(), String::new()));

    let semmap_same = SemmapFile {
        project_name: "test".into(),
        purpose: String::new(),
        legend: vec![],
        layers: vec![layer2_same],
    };

    let violations_same = deps::check_layer_violations(&depmap_same, &semmap_same);
    assert!(
        violations_same.is_empty(),
        "Same layer should not be violation"
    );

    // Test case 2: Downward dependency (L2 depends on L3) - VIOLATION
    let depmap_down = DependencyMap {
        nodes: vec![
            DepNode {
                path: "low.rs".into(),
                layer: 2,
            },
            DepNode {
                path: "high.rs".into(),
                layer: 3,
            },
        ],
        edges: vec![DepEdge {
            from: "low.rs".into(),
            to: "high.rs".into(),
            kind: DepKind::Import,
        }],
    };

    let mut layer2_down = Layer::new(2, "Domain".into());
    let mut layer3_down = Layer::new(3, "Utils".into());
    layer2_down.entries.push(FileEntry::new(
        "low.rs".into(),
        "desc".into(),
        String::new(),
    ));
    layer3_down.entries.push(FileEntry::new(
        "high.rs".into(),
        "desc".into(),
        String::new(),
    ));

    let semmap_down = SemmapFile {
        project_name: "test".into(),
        purpose: String::new(),
        legend: vec![],
        layers: vec![layer2_down, layer3_down],
    };

    let violations_down = deps::check_layer_violations(&depmap_down, &semmap_down);
    assert!(!violations_down.is_empty(), "L2 -> L3 should be violation");

    // Safe access without indexing
    let first_violation = violations_down.first().unwrap();
    assert!(first_violation.contains("L2") && first_violation.contains("L3"));
}

#[test]
fn layer_violation_ignores_facade_files() {
    let depmap = DependencyMap {
        nodes: vec![
            DepNode {
                path: "src/lib.rs".into(),
                layer: 1,
            },
            DepNode {
                path: "src/utils.rs".into(),
                layer: 3,
            },
        ],
        edges: vec![DepEdge {
            from: "src/lib.rs".into(),
            to: "src/utils.rs".into(),
            kind: DepKind::Import,
        }],
    };

    let mut layer1 = Layer::new(1, "Core".into());
    let mut layer3 = Layer::new(3, "Utils".into());
    layer1.entries.push(FileEntry::new(
        "src/lib.rs".into(),
        "desc".into(),
        String::new(),
    ));
    layer3.entries.push(FileEntry::new(
        "src/utils.rs".into(),
        "desc".into(),
        String::new(),
    ));

    let semmap = SemmapFile {
        project_name: "test".into(),
        purpose: String::new(),
        legend: vec![],
        layers: vec![layer1, layer3],
    };

    let violations = deps::check_layer_violations(&depmap, &semmap);
    assert!(
        violations.is_empty(),
        "lib.rs facade should not trigger violation"
    );
}
