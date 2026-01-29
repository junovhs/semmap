//! Extracts documentation comments from source files.

/// Extract the primary doc comment from a Rust file.
/// Returns module-level `//!` comment if present, else first `///` block.
pub fn extract_doc_comment(content: &str) -> Option<String> {
    extract_module_doc(content).or_else(|| extract_first_item_doc(content))
}

/// Extract module-level documentation (//! comments at file start).
fn extract_module_doc(content: &str) -> Option<String> {
    let mut doc_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("//!") {
            doc_lines.push(rest.trim());
        } else if !trimmed.is_empty() && !trimmed.starts_with("//") {
            break;
        }
    }

    if doc_lines.is_empty() {
        None
    } else {
        Some(collapse_doc_lines(&doc_lines))
    }
}

/// Extract the first `///` doc comment block.
fn extract_first_item_doc(content: &str) -> Option<String> {
    let mut doc_lines = Vec::new();
    let mut in_doc_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("///") {
            if !rest.starts_with('/') {
                doc_lines.push(rest.trim());
                in_doc_block = true;
            }
        } else if in_doc_block {
            break;
        }
    }

    if doc_lines.is_empty() {
        None
    } else {
        Some(collapse_doc_lines(&doc_lines))
    }
}

/// Collapse multiple doc lines into a single sentence.
fn collapse_doc_lines(lines: &[&str]) -> String {
    let joined = lines.join(" ");
    let trimmed = joined.trim();

    if let Some(idx) = trimmed.find(". ") {
        return format!("{}.", &trimmed[..idx]);
    }

    if trimmed.ends_with('.') {
        trimmed.to_string()
    } else {
        format!("{trimmed}.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_doc() {
        let content = "//! This module handles parsing.\n\nuse std::io;";
        assert_eq!(
            extract_doc_comment(content),
            Some("This module handles parsing.".into())
        );
    }

    #[test]
    fn test_item_doc() {
        let content = "use std::io;\n\n/// Parses the input string.\npub fn parse() {}";
        assert_eq!(
            extract_doc_comment(content),
            Some("Parses the input string.".into())
        );
    }

    #[test]
    fn test_no_doc() {
        let content = "use std::io;\npub fn parse() {}";
        assert_eq!(extract_doc_comment(content), None);
    }
}
