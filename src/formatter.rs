use crate::types::SemmapFile;

pub fn to_markdown(semmap: &SemmapFile) -> String {
    let mut out = String::new();

    write_header(&mut out, semmap);
    write_legend(&mut out, semmap);
    write_layers(&mut out, semmap);

    out
}

fn write_header(out: &mut String, semmap: &SemmapFile) {
    out.push_str(&format!("# {} — Semantic Map\n\n", semmap.project_name));

    if !semmap.purpose.is_empty() {
        out.push_str(&format!("**Purpose:** {}\n\n", semmap.purpose));
    }
}

fn write_legend(out: &mut String, semmap: &SemmapFile) {
    if semmap.legend.is_empty() {
        return;
    }

    out.push_str("## Legend\n\n");

    for entry in &semmap.legend {
        out.push_str(&format!("`[{}]` {}\n\n", entry.tag, entry.definition));
    }
}

fn write_layers(out: &mut String, semmap: &SemmapFile) {
    for layer in &semmap.layers {
        out.push_str(&format!("## Layer {} — {}\n\n", layer.number, layer.name));

        for entry in &layer.entries {
            out.push_str(&format!("`{}`\n", entry.path));
            
            let desc = format_description(&entry.description);
            out.push_str(&format!("{}\n", desc));

            if let Some(exports) = &entry.exports {
                if !exports.is_empty() {
                    out.push_str(&format!("→ Exports: {}\n", exports.join(", ")));
                }
            }

            if let Some(touch) = &entry.touch {
                out.push_str(&format!("→ Touch: {}\n", touch));
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
