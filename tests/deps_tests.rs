use semmap::deps::{check_layer_violations, render_mermaid};
use semmap::types::{
    DepEdge, DepKind, DepNode, DependencyMap, Description, FileEntry, Layer, SemmapFile,
};

fn entry(path: &str) -> FileEntry {
    FileEntry {
        path: path.to_string(),
        description: Description {
            what: "Test.".to_string(),
            why: "Test.".to_string(),
        },
        exports: None,
        touch: None,
    }
}

fn layer(number: u8, paths: &[&str]) -> Layer {
    Layer {
        number,
        name: format!("Layer{number}"),
        entries: paths.iter().map(|p| entry(p)).collect(),
    }
}

/// Lower layer depending on higher layer should be flagged as violation.
#[test]
fn test_layer_violation_detected() {
    let semmap = SemmapFile {
        project_name: "test".to_string(),
        purpose: "Test.".to_string(),
        legend: vec![],
        layers: vec![
            layer(0, &["config.rs"]),
            layer(1, &["core.rs"]),
            layer(2, &["app.rs"]),
        ],
    };

    let depmap = DependencyMap {
        nodes: vec![
            DepNode {
                path: "config.rs".to_string(),
                layer: 0,
            },
            DepNode {
                path: "core.rs".to_string(),
                layer: 1,
            },
            DepNode {
                path: "app.rs".to_string(),
                layer: 2,
            },
        ],
        edges: vec![
            // Layer 0 depending on Layer 2 = violation
            DepEdge {
                from: "config.rs".to_string(),
                to: "app.rs".to_string(),
                kind: DepKind::Import,
            },
        ],
    };

    let violations = check_layer_violations(&depmap, &semmap);
    assert_eq!(violations.len(), 1);
    assert!(violations
        .first()
        .is_some_and(|v| v.contains("config.rs") && v.contains("app.rs")));
}

/// Higher layer depending on lower layer is valid (no violation).
#[test]
fn test_valid_dependency_no_violation() {
    let semmap = SemmapFile {
        project_name: "test".to_string(),
        purpose: "Test.".to_string(),
        legend: vec![],
        layers: vec![layer(0, &["low.rs"]), layer(1, &["high.rs"])],
    };

    let depmap = DependencyMap {
        nodes: vec![
            DepNode {
                path: "low.rs".to_string(),
                layer: 0,
            },
            DepNode {
                path: "high.rs".to_string(),
                layer: 1,
            },
        ],
        edges: vec![
            // Layer 1 depending on Layer 0 = valid
            DepEdge {
                from: "high.rs".to_string(),
                to: "low.rs".to_string(),
                kind: DepKind::Import,
            },
        ],
    };

    let violations = check_layer_violations(&depmap, &semmap);
    assert!(violations.is_empty());
}

/// Same layer dependencies are valid.
#[test]
fn test_same_layer_no_violation() {
    let semmap = SemmapFile {
        project_name: "test".to_string(),
        purpose: "Test.".to_string(),
        legend: vec![],
        layers: vec![layer(1, &["a.rs", "b.rs"])],
    };

    let depmap = DependencyMap {
        nodes: vec![
            DepNode {
                path: "a.rs".to_string(),
                layer: 1,
            },
            DepNode {
                path: "b.rs".to_string(),
                layer: 1,
            },
        ],
        edges: vec![DepEdge {
            from: "a.rs".to_string(),
            to: "b.rs".to_string(),
            kind: DepKind::Import,
        }],
    };

    let violations = check_layer_violations(&depmap, &semmap);
    assert!(violations.is_empty());
}

/// Mermaid output should start with graph TD header.
#[test]
fn test_render_mermaid_header() {
    let depmap = DependencyMap {
        nodes: vec![],
        edges: vec![],
    };
    let output = render_mermaid(&depmap);
    assert!(output.starts_with("graph TD"));
}

/// Mermaid should include node definitions with file names as labels.
#[test]
fn test_render_mermaid_nodes() {
    let depmap = DependencyMap {
        nodes: vec![DepNode {
            path: "src/lib.rs".to_string(),
            layer: 0,
        }],
        edges: vec![],
    };
    let output = render_mermaid(&depmap);
    assert!(output.contains("lib.rs"), "Should use filename as label");
}

/// Mermaid should include edge definitions with arrows.
#[test]
fn test_render_mermaid_edges() {
    let depmap = DependencyMap {
        nodes: vec![
            DepNode {
                path: "a.rs".to_string(),
                layer: 0,
            },
            DepNode {
                path: "b.rs".to_string(),
                layer: 0,
            },
        ],
        edges: vec![DepEdge {
            from: "a.rs".to_string(),
            to: "b.rs".to_string(),
            kind: DepKind::Import,
        }],
    };
    let output = render_mermaid(&depmap);
    assert!(output.contains("-->"), "Should have import arrow");
}
