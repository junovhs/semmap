use semmap::parser;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_parse_valid_simple_semmap() -> TestResult {
    let content = r"# My Project -- Semantic Map
**Purpose:** To take over the world.

## Layer 0 -- Config
`Cargo.toml`
Defines dependencies. Needed for build.
";
    let result = parser::parse(content)?;
    assert_eq!(result.project_name, "My Project");
    assert_eq!(result.purpose, "To take over the world.");
    assert_eq!(result.layers.len(), 1);
    
    let layer = result.layers.first().ok_or("Missing layer 0")?;
    assert_eq!(layer.number, 0);
    assert_eq!(layer.name, "Config");
    assert_eq!(layer.entries.len(), 1);
    
    let entry = layer.entries.first().ok_or("Missing entry 0")?;
    assert_eq!(entry.path, "Cargo.toml");
    assert_eq!(entry.description.what, "Defines dependencies.");
    assert_eq!(entry.description.why, "Needed for build.");
    Ok(())
}

#[test]
fn test_parse_missing_title() {
    let content = r"**Purpose:** Missing title.
";
    let result = parser::parse(content);
    assert!(result.is_err());
}

#[test]
fn test_parse_multiple_layers() -> TestResult {
    let content = r"# Multi -- Semantic Map
Purpose: Testing layers.

## Layer 1 -- Core
`src/lib.rs`
Core logic. Base.

## Layer 2 -- Network
`src/net.rs`
Network logic. Connects to internet.
";
    let result = parser::parse(content)?;
    assert_eq!(result.layers.len(), 2);
    
    let l1 = result.layers.first().ok_or("Missing layer 1")?;
    assert_eq!(l1.number, 1);
    assert_eq!(l1.name, "Core");
    
    let l2 = result.layers.get(1).ok_or("Missing layer 2")?;
    assert_eq!(l2.number, 2);
    assert_eq!(l2.name, "Network");
    Ok(())
}

#[test]
fn test_parse_exports_and_touch() -> TestResult {
    let content = r"# Meta -- Semantic Map
Purpose: Meta test.

## Layer 0 -- Config
`src/foo.rs`
Does foo.
  Exports: Bar, Baz
  Touch: Careful with this.
";
    let result = parser::parse(content)?;
    let layer = result.layers.first().ok_or("Missing layer")?;
    let entry = layer.entries.first().ok_or("Missing entry")?;
    
    assert!(entry.exports.is_some());
    let exports = entry.exports.as_ref().ok_or("Missing exports")?;
    assert_eq!(exports.len(), 2);
    
    assert_eq!(exports.first().map(String::as_str), Some("Bar"));
    assert_eq!(exports.get(1).map(String::as_str), Some("Baz"));
    
    assert!(entry.touch.is_some());
    assert_eq!(entry.touch.as_deref(), Some("Careful with this."));
    Ok(())
}

#[test]
fn test_parse_description_splitting() -> TestResult {
    let content = r"# Split -- Semantic Map
Purpose: Split test.

## Layer 0 -- Split
`file1`
Only what.

`file2`
What part. Why part.
";
    let result = parser::parse(content)?;
    let layer = result.layers.first().ok_or("Missing layer")?;
    
    // file1
    let f1 = layer.entries.first().ok_or("Missing f1")?;
    assert_eq!(f1.description.what, "Only what."); 
    assert!(f1.description.why.is_empty());
    
    // file2
    let f2 = layer.entries.get(1).ok_or("Missing f2")?;
    assert_eq!(f2.description.what, "What part.");
    assert_eq!(f2.description.why, "Why part.");
    Ok(())
}

#[test]
fn test_parse_legend() -> TestResult {
     let content = r"# Legend Test -- Semantic Map
Purpose: To test legend parsing.

## Legend
`[CRITICAL]` High importance.
`[TODO]` Needs work.

## Layer 0
`foo`
bar. baz.
";
    let result = parser::parse(content)?;
    assert_eq!(result.legend.len(), 2);
    assert_eq!(result.legend.first().map(|x| x.tag.as_str()), Some("CRITICAL"));
    assert_eq!(result.legend.first().map(|x| x.definition.as_str()), Some("High importance."));
    assert_eq!(result.legend.get(1).map(|x| x.tag.as_str()), Some("TODO"));
    assert_eq!(result.legend.get(1).map(|x| x.definition.as_str()), Some("Needs work."));
    Ok(())
}
