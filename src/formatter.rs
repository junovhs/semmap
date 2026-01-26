use crate::types::SemmapFile;
use std::fmt::Write;

pub fn to_markdown(semmap: &SemmapFile) -> String {
    let mut out = String::new();

    write_header(&mut out, semmap);
    write_legend(&mut out, semmap);
    write_layers(&mut out, semmap);

    out
}

fn write_header(out: &mut String, semmap: &SemmapFile) {
    let _ = writeln!(out, "# {} — Semantic Map\n", semmap.project_name);

    if !semmap.purpose.is_empty() {
        let _ = writeln!(out, "**Purpose:** {}\n", semmap.purpose);
    }
}

fn write_legend(out: &mut String, semmap: &SemmapFile) {
    if semmap.legend.is_empty() {
        return;
    }

    out.push_str("## Legend\n\n");

    for entry in &semmap.legend {
        let _ = writeln!(out, "`[{}]` {}\n", entry.tag, entry.definition);
    }
}

fn write_layers(out: &mut String, semmap: &SemmapFile) {
    for layer in &semmap.layers {
        let _ = writeln!(out, "## Layer {} — {}\n", layer.number, layer.name);

        for entry in &layer.entries {
            let _ = writeln!(out, "`{}`", entry.path);
            
            let desc = format_description(&entry.description);
            let _ = writeln!(out, "{desc}");

            if let Some(exports) = &entry.exports {
                if !exports.is_empty() {
                    let _ = writeln!(out, "→ Exports: {}", exports.join(", "));
                }
            }

            if let Some(touch) = &entry.touch {
                let _ = writeln!(out, "→ Touch: {touch}");
            }

            out.push('\n');
        }
    }
}

fn format_description(desc: &crate::types::Description) -> String {
    if desc.why.is_empty() {
        desc.what.clone()
    } else {
        format!("{} {}", desc.what, desc.why)
    }
}

pub fn to_json(semmap: &SemmapFile) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(semmap)
}

pub fn to_toml(semmap: &SemmapFile) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(semmap)
}
