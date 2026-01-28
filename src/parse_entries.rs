use crate::types::{Description, FileEntry};
use regex::Regex;
use std::sync::OnceLock;

static PATH_RE: OnceLock<Option<Regex>> = OnceLock::new();

fn path_regex() -> Option<&'static Regex> {
    PATH_RE.get_or_init(|| {
        Regex::new(r"^`([^`]+)`")
            .or_else(|_| Regex::new(r"^(\S+\.\w+)"))
            .ok()
    }).as_ref()
}

pub fn parse_layer_entries(lines: &[&str], idx: &mut usize) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let Some(path_re) = path_regex() else {
        return entries;
    };

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };

        if line.starts_with("## Layer") || line.starts_with("# ") {
            break;
        }

        if let Some(caps) = path_re.captures(line) {
            let path = caps.get(1).map_or(String::new(), |m| m.as_str().into());
            *idx += 1;
            let entry = parse_file_entry(path, lines, idx);
            entries.push(entry);
        } else {
            *idx += 1;
        }
    }

    entries
}

fn parse_file_entry(path: String, lines: &[&str], idx: &mut usize) -> FileEntry {
    let mut desc_parts: Vec<&str> = Vec::new();
    let mut exports = None;
    let mut touch = None;

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('`') || trimmed.starts_with("## ") {
            break;
        }

        if let Some(rest) = trimmed.strip_prefix("Exports:") {
            exports = Some(parse_exports(rest));
        } else if let Some(rest) = trimmed.strip_prefix("Touch:") {
            touch = Some(rest.trim().into());
        } else {
            desc_parts.push(trimmed);
        }

        *idx += 1;
    }

    let full_desc = desc_parts.join(" ");
    let (what, why) = split_description(&full_desc);

    FileEntry {
        path,
        description: Description { what, why },
        exports,
        touch,
    }
}

fn parse_exports(rest: &str) -> Vec<String> {
    rest.trim().split(',').map(|s| s.trim().into()).collect()
}

fn split_description(desc: &str) -> (String, String) {
    match desc.split_once(". ") {
        Some((first, rest)) => (format!("{first}."), rest.into()),
        None => (desc.into(), String::new()),
    }
}
