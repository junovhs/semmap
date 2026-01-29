//! Layer and description inference for SEMMAP generation.

use crate::doc_extractor;
use crate::stereotype::{self, Stereotype};
use crate::swum;
use std::path::Path;

/// Infer the layer number for a file based on its path and content.
pub fn infer_layer(rel_path: &str, file: &Path, content: &str) -> u8 {
    let stereotype = stereotype::classify(rel_path, content);

    match stereotype {
        Stereotype::Config => 0,
        Stereotype::Entrypoint | Stereotype::Cli => 1,
        Stereotype::Entity
        | Stereotype::Parser
        | Stereotype::Formatter
        | Stereotype::Service
        | Stereotype::Error => 2,
        Stereotype::Utility | Stereotype::Repository | Stereotype::Handler => 3,
        Stereotype::Test => 4,
        Stereotype::Unknown => infer_layer_from_path(rel_path, file),
    }
}

/// Fallback layer inference from path patterns.
fn infer_layer_from_path(rel_path: &str, file: &Path) -> u8 {
    let lower = rel_path.to_lowercase();
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");

    if is_config_ext(ext) {
        return 0;
    }

    if lower.contains("main") || lower.contains("lib.rs") || lower.contains("mod.rs") {
        return 1;
    }

    if lower.contains("types") || lower.contains("model") || lower.contains("schema") {
        return 2;
    }

    if lower.contains("util") || lower.contains("helper") || lower.contains("common") {
        return 3;
    }

    if lower.contains("test") || lower.contains("spec") {
        return 4;
    }

    2
}

fn is_config_ext(ext: &str) -> bool {
    matches!(ext, "toml" | "yaml" | "yml" | "json")
}

/// Infer the WHAT description for a file.
/// Priority: doc comment > SWUM expansion > generic fallback.
pub fn infer_what(rel_path: &str, file: &Path, content: &str) -> String {
    // Try doc comment first
    if let Some(doc) = doc_extractor::extract_doc_comment(content) {
        return doc;
    }

    // Try SWUM expansion on file stem
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("file");

    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Special cases with good defaults
    match ext {
        "rs" if rel_path.contains("main") => return "Application entry point.".into(),
        "rs" if rel_path.contains("lib") => return "Library root and public exports.".into(),
        "rs" if rel_path.contains("mod") => return format!("Module definitions for {stem}."),
        "toml" if stem == "Cargo" => return "Rust package manifest and dependencies.".into(),
        "json" if stem == "package" => return "Node.js package manifest.".into(),
        _ => {}
    }

    // Use SWUM for code files
    if matches!(ext, "rs" | "py" | "ts" | "js" | "go" | "java") {
        return swum::expand_identifier(stem);
    }

    // Config files
    if matches!(ext, "toml" | "yaml" | "yml" | "json") {
        return format!("Configuration for {stem}.");
    }

    format!("Handles {stem}.")
}

/// Infer the WHY description for a file based on its stereotype.
pub fn infer_why(rel_path: &str, content: &str) -> String {
    let stereotype = stereotype::classify(rel_path, content);
    stereotype::stereotype_to_why(stereotype).to_string()
}
