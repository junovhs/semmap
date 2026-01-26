use crate::error::{ParseError, SemmapError};
use crate::types::{Description, FileEntry, Layer, LegendEntry, SemmapFile};
use regex::Regex;

pub fn parse(content: &str) -> Result<SemmapFile, SemmapError> {
    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0;

    let (project_name, purpose) = parse_header(&lines, &mut idx)?;
    let legend = parse_legend(&lines, &mut idx)?;
    let layers = parse_layers(&lines, &mut idx)?;

    Ok(SemmapFile {
        project_name,
        purpose,
        legend,
        layers,
    })
}

fn parse_header(lines: &[&str], idx: &mut usize) -> Result<(String, String), SemmapError> {
    let title_re = Regex::new(r"^#\s+(.+?)\s*[—-]\s*Semantic Map").unwrap_or_else(|_| {
        Regex::new(r"^#\s+(.+)").expect("fallback regex")
    });
    
    let purpose_re = Regex::new(r"\*\*Purpose:\*\*\s*(.+)").unwrap_or_else(|_| {
        Regex::new(r"Purpose:\s*(.+)").expect("fallback regex")
    });

    let mut project_name = String::new();
    let mut purpose = String::new();

    while *idx < lines.len() {
        let line = lines[*idx];
        
        if let Some(caps) = title_re.captures(line) {
            project_name = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        }
        
        if let Some(caps) = purpose_re.captures(line) {
            purpose = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
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

fn parse_legend(lines: &[&str], idx: &mut usize) -> Result<Vec<LegendEntry>, SemmapError> {
    let mut legend = Vec::new();
    let entry_re = Regex::new(r"`\[([A-Z]+)\]`\s+(.+)").unwrap_or_else(|_| {
        Regex::new(r"\[([A-Z]+)\]\s+(.+)").expect("fallback")
    });

    while *idx < lines.len() {
        let line = lines[*idx];
        
        if line.starts_with("## Layer") {
            break;
        }

        if let Some(caps) = entry_re.captures(line) {
            let tag = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let def = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            legend.push(LegendEntry { tag, definition: def });
        }

        *idx += 1;
    }

    Ok(legend)
}

fn parse_layers(lines: &[&str], idx: &mut usize) -> Result<Vec<Layer>, SemmapError> {
    let mut layers = Vec::new();
    let layer_re = Regex::new(r"^##\s+Layer\s+(\d+)\s*[—-]\s*(.+)").unwrap_or_else(|_| {
        Regex::new(r"^##\s+Layer\s+(\d+)").expect("fallback")
    });

    while *idx < lines.len() {
        let line = lines[*idx];

        if let Some(caps) = layer_re.captures(line) {
            let num: u8 = caps.get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let name = caps.get(2)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            
            *idx += 1;
            let entries = parse_layer_entries(lines, idx)?;
            layers.push(Layer { number: num, name, entries });
        } else {
            *idx += 1;
        }
    }

    Ok(layers)
}

fn parse_layer_entries(lines: &[&str], idx: &mut usize) -> Result<Vec<FileEntry>, SemmapError> {
    let mut entries = Vec::new();
    let path_re = Regex::new(r"^`([^`]+)`").unwrap_or_else(|_| {
        Regex::new(r"^(\S+\.\w+)").expect("fallback")
    });

    while *idx < lines.len() {
        let line = lines[*idx];

        if line.starts_with("## Layer") || line.starts_with("# ") {
            break;
        }

        if let Some(caps) = path_re.captures(line) {
            let path = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            *idx += 1;
            let entry = parse_file_entry(path, lines, idx)?;
            entries.push(entry);
        } else {
            *idx += 1;
        }
    }

    Ok(entries)
}

fn parse_file_entry(path: String, lines: &[&str], idx: &mut usize) -> Result<FileEntry, SemmapError> {
    let mut description_parts = Vec::new();
    let mut exports = None;
    let mut touch = None;

    while *idx < lines.len() {
        let line = lines[*idx];
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('`') || trimmed.starts_with("## ") {
            break;
        }

        if trimmed.starts_with("→ Exports:") {
            let items = trimmed.trim_start_matches("→ Exports:").trim();
            exports = Some(items.split(',').map(|s| s.trim().to_string()).collect());
        } else if trimmed.starts_with("→ Touch:") {
            touch = Some(trimmed.trim_start_matches("→ Touch:").trim().to_string());
        } else {
            description_parts.push(trimmed.to_string());
        }

        *idx += 1;
    }

    let full_desc = description_parts.join(" ");
    let (what, why) = split_description(&full_desc);

    Ok(FileEntry {
        path,
        description: Description { what, why },
        exports,
        touch,
    })
}

fn split_description(desc: &str) -> (String, String) {
    let sentences: Vec<&str> = desc.split(". ").collect();
    if sentences.len() >= 2 {
        let what = format!("{}.", sentences[0]);
        let why = sentences[1..].join(". ");
        (what, why)
    } else {
        (desc.to_string(), String::new())
    }
}
