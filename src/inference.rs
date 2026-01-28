use std::path::Path;

pub fn infer_layer(rel_path: &str, file: &Path) -> u8 {
    let lower = rel_path.to_lowercase();
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");

    if is_config_file(&lower, ext) {
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

fn is_config_file(path: &str, ext: &str) -> bool {
    let config_exts = ["toml", "yaml", "yml", "json"];
    let config_names = ["config", "settings", "cargo", "package", "tsconfig"];
    
    config_exts.contains(&ext) || config_names.iter().any(|n| path.contains(n))
}

pub fn infer_what(rel_path: &str, file: &Path) -> String {
    let stem = file.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");

    let ext = file.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" if rel_path.contains("main") => "Application entry point.".into(),
        "rs" if rel_path.contains("lib") => "Library root and public exports.".into(),
        "rs" if rel_path.contains("mod") => format!("Module definitions for {stem}."),
        "rs" => format!("Implements {stem} functionality."),
        "toml" if stem == "Cargo" => "Rust package manifest and dependencies.".into(),
        "json" if stem == "package" => "Node.js package manifest.".into(),
        "json" | "yaml" | "yml" => format!("Configuration for {stem}."),
        _ => format!("Handles {stem}."),
    }
}

pub fn infer_why(rel_path: &str) -> String {
    if rel_path.contains("config") || rel_path.contains("Cargo") {
        return "Centralizes build and runtime configuration.".into();
    }
    
    if rel_path.contains("types") || rel_path.contains("model") {
        return "Isolates data structures for reuse across modules.".into();
    }

    if rel_path.contains("error") {
        return "Provides unified error handling.".into();
    }

    "Separates concerns for maintainability.".into()
}