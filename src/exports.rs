use std::fs;
use std::path::Path;

pub fn extract_exports(file: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(file).ok()?;
    let mut exports = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        
        if let Some(name) = try_extract_pub_item(trimmed) {
            exports.push(name);
        }
    }

    if exports.is_empty() { None } else { Some(exports) }
}

fn try_extract_pub_item(line: &str) -> Option<String> {
    if line.starts_with("pub struct ") {
        return extract_ident(line, "pub struct ");
    }
    if line.starts_with("pub fn ") {
        return extract_ident(line, "pub fn ");
    }
    if line.starts_with("pub trait ") {
        return extract_ident(line, "pub trait ");
    }
    if line.starts_with("pub enum ") {
        return extract_ident(line, "pub enum ");
    }
    None
}

fn extract_ident(line: &str, prefix: &str) -> Option<String> {
    let rest = line.strip_prefix(prefix)?;
    let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_')?;
    Some(rest.get(..end)?.to_string())
}