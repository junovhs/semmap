use crate::error::{ParseError, SemmapError};
use crate::types::{Description, FileEntry, Layer, LegendEntry, SemmapFile};
use regex::Regex;
use std::sync::OnceLock;

static TITLE_RE: OnceLock<Option<Regex>> = OnceLock::new();
static PURPOSE_RE: OnceLock<Option<Regex>> = OnceLock::new();
static LEGEND_RE: OnceLock<Option<Regex>> = OnceLock::new();
static LAYER_RE: OnceLock<Option<Regex>> = OnceLock::new();
static PATH_RE: OnceLock<Option<Regex>> = OnceLock::new();

fn title_regex() -> Option<&'static Regex> {
    TITLE_RE.get_or_init(|| {
        Regex::new(r"^#\s+(.+?)\s*[—-]\s*Semantic Map")
            .or_else(|_| Regex::new(r"^#\s+(.+)"))
            .ok()
    }).as_ref()
}

fn purpose_regex() -> Option<&'static Regex> {
    PURPOSE_RE.get_or_init(|| {
        Regex::new(r"\*\*Purpose:\*\*\s*(.+)")
            .or_else(|_| Regex::new(r"Purpose:\s*(.+)"))
            .ok()
    }).as_ref()
}

fn legend_regex() -> Option<&'static Regex> {
    LEGEND_RE.get_or_init(|| {
        Regex::new(r"`\[([A-Z]+)\]`\s+(.+)")
            .or_else(|_| Regex::new(r"\[([A-Z]+)\]\s+(.+)"))
            .ok()
    }).as_ref()
}

fn layer_regex() -> Option<&'static Regex> {
    LAYER_RE.get_or_init(|| {
        Regex::new(r"^##\s+Layer\s+(\d+)\s*[—-]\s*(.+)")
            .or_else(|_| Regex::new(r"^##\s+Layer\s+(\d+)"))
            .ok()
    }).as_ref()
}

fn path_regex() -> Option<&'static Regex> {
    PATH_RE.get_or_init(|| {
        Regex::new(r"^`([^`]+)`")
            .or_else(|_| Regex::new(r"^(\S+\.\w+)"))
            .ok()
    }).as_ref()
}

pub fn parse(content: &str) -> Result<SemmapFile, SemmapError> {
    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0;

    let (project_name, purpose) = parse_header(&lines, &mut idx)?;
    let legend = parse_legend(&lines, &mut idx);
    let layers = parse_layers(&lines, &mut idx);

    Ok(SemmapFile {
        project_name,
        purpose,
        legend,
        layers,
    })
}

fn parse_header(lines: &[&str], idx: &mut usize) -> Result<(String, String), SemmapError> {
    let mut project_name = String::new();
    let mut purpose = String::new();

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };
        
        if let Some(re) = title_regex() {
            if let Some(caps) = re.captures(line) {
                if let Some(m) = caps.get(1) {
                    project_name = m.as_str().into();
                }
            }
        }
        
        if let Some(re) = purpose_regex() {
            if let Some(caps) = re.captures(line) {
                if let Some(m) = caps.get(1) {
                    purpose = m.as_str().into();
                }
            }
        }

        if line.starts_with("## Legend") {
            break;
        }
        
        *idx += 1;
    }

    if project_name.is_empty() {
        return Err(SemmapError::Parse(ParseError {
            line: 1,
            message: "Missing project title (# name — Semantic Map)".into(),
        }));
    }

    Ok((project_name, purpose))
}

fn parse_legend(lines: &[&str], idx: &mut usize) -> Vec<LegendEntry> {
    let mut legend = Vec::new();
    let Some(entry_re) = legend_regex() else {
        return legend;
    };

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };
        
        if line.starts_with("## Layer") {
            break;
        }

        if let Some(caps) = entry_re.captures(line) {
            let tag = caps.get(1).map_or(String::new(), |m| m.as_str().into());
            let definition = caps.get(2).map_or(String::new(), |m| m.as_str().into());
            legend.push(LegendEntry { tag, definition });
        }

        *idx += 1;
    }

    legend
}

fn parse_layers(lines: &[&str], idx: &mut usize) -> Vec<Layer> {
    let mut layers = Vec::new();
    let Some(layer_re) = layer_regex() else {
        return layers;
    };

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };

        if let Some(caps) = layer_re.captures(line) {
            let num: u8 = caps.get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let name = caps.get(2)
                .map_or(String::new(), |m| m.as_str().trim().into());
            
            *idx += 1;
            let entries = parse_layer_entries(lines, idx);
            layers.push(Layer { number: num, name, entries });
        } else {
            *idx += 1;
        }
    }

    layers
}

fn parse_layer_entries(lines: &[&str], idx: &mut usize) -> Vec<FileEntry> {
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

        if let Some(rest) = trimmed.strip_prefix("→ Exports:") {
            exports = Some(rest.trim().split(',').map(|s| s.trim().into()).collect());
        } else if let Some(rest) = trimmed.strip_prefix("→ Touch:") {
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

fn split_description(desc: &str) -> (String, String) {
    match desc.split_once(". ") {
        Some((first, rest)) => (format!("{first}."), rest.into()),
        None => (desc.into(), String::new()),
    }
}
