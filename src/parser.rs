use crate::error::{ParseError, SemmapError};
use crate::parse_entries;
use crate::types::{Layer, LegendEntry, SemmapFile};
use regex::Regex;
use std::sync::OnceLock;

static TITLE_RE: OnceLock<Option<Regex>> = OnceLock::new();
static PURPOSE_RE: OnceLock<Option<Regex>> = OnceLock::new();
static LEGEND_RE: OnceLock<Option<Regex>> = OnceLock::new();
static LAYER_RE: OnceLock<Option<Regex>> = OnceLock::new();

fn title_regex() -> Option<&'static Regex> {
    TITLE_RE.get_or_init(|| {
        Regex::new(r"^#\s+(.+?)\s*[--]\s*Semantic Map")
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
        Regex::new(r"^##\s+Layer\s+(\d+)\s*[--]\s*(.+)")
            .or_else(|_| Regex::new(r"^##\s+Layer\s+(\d+)"))
            .ok()
    }).as_ref()
}

pub fn parse(content: &str) -> Result<SemmapFile, SemmapError> {
    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0;

    let (project_name, purpose) = parse_header(&lines, &mut idx)?;
    let legend = parse_legend(&lines, &mut idx);
    let layers = parse_layers(&lines, &mut idx);

    Ok(SemmapFile { project_name, purpose, legend, layers })
}

fn parse_header(lines: &[&str], idx: &mut usize) -> Result<(String, String), SemmapError> {
    let mut project_name = String::new();
    let mut purpose = String::new();

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };
        
        if project_name.is_empty() {
            project_name = try_extract_title(line);
        }
        
        if purpose.is_empty() {
            purpose = try_extract_purpose(line);
        }

        if line.starts_with("## Legend") || line.starts_with("## Layer") {
            break;
        }
        
        *idx += 1;
    }

    if project_name.is_empty() {
        return Err(SemmapError::Parse(ParseError {
            line: 1,
            message: "Missing project title (# name - Semantic Map)".into(),
        }));
    }

    Ok((project_name, purpose))
}

fn try_extract_title(line: &str) -> String {
    let Some(re) = title_regex() else { return String::new() };
    re.captures(line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().into())
        .unwrap_or_default()
}

fn try_extract_purpose(line: &str) -> String {
    let Some(re) = purpose_regex() else { return String::new() };
    re.captures(line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().into())
        .unwrap_or_default()
}

fn parse_legend(lines: &[&str], idx: &mut usize) -> Vec<LegendEntry> {
    let mut legend = Vec::new();
    let Some(entry_re) = legend_regex() else { return legend };

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
    let Some(layer_re) = layer_regex() else { return layers };

    while *idx < lines.len() {
        let Some(&line) = lines.get(*idx) else { break };

        if let Some(caps) = layer_re.captures(line) {
            let num: u8 = caps.get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let name = caps.get(2)
                .map_or(String::new(), |m| m.as_str().trim().into());
            
            *idx += 1;
            let entries = parse_entries::parse_layer_entries(lines, idx);
            layers.push(Layer { number: num, name, entries });
        } else {
            *idx += 1;
        }
    }

    layers
}