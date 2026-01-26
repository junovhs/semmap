use crate::types::{FileEntry, Layer, LegendEntry, SemmapFile};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct GeneratorConfig {
    pub project_name: String,
    pub purpose: String,
    pub include_exts: Vec<String>,
    pub exclude_dirs: Vec<String>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            purpose: String::new(),
            include_exts: vec![
                "rs", "ts", "js", "py", "go", "java", "toml", "yaml", "json"
            ].into_iter().map(String::from).collect(),
            exclude_dirs: vec![
                ".git", "target", "node_modules", "dist", "build", "__pycache__"
            ].into_iter().map(String::from).collect(),
        }
    }
}

pub fn generate(root: &Path, config: GeneratorConfig) -> SemmapFile {
    let files = collect_files(root, &config);
    let classified = classify_by_layer(&files, root);
    
    let project_name = if config.project_name.is_empty() {
        root.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".into())
    } else {
        config.project_name
    };

    let mut semmap = SemmapFile::new(project_name, config.purpose);
    semmap.legend = default_legend();
    semmap.layers = build_layers(classified, root);

    semmap
}

fn collect_files(root: &Path, config: &GeneratorConfig) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_excluded(e, &config.exclude_dirs));

    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let matches_ext = entry.path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| config.include_exts.iter().any(|inc| inc == e))
            .unwrap_or(false);

        if matches_ext {
            files.push(entry.path().to_path_buf());
        }
    }

    files
}

fn is_excluded(entry: &walkdir::DirEntry, excludes: &[String]) -> bool {
    let name = entry.file_name().to_string_lossy();
    name.starts_with('.') || excludes.iter().any(|ex| ex == name.as_ref())
}

fn classify_by_layer(files: &[std::path::PathBuf], root: &Path) -> HashMap<u8, Vec<FileEntry>> {
    let mut layers: HashMap<u8, Vec<FileEntry>> = HashMap::new();

    for file in files {
        let rel_path = file.strip_prefix(root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let layer = infer_layer(&rel_path, file);
        let entry = create_entry(&rel_path, file);

        layers.entry(layer).or_default().push(entry);
    }

    layers
}

fn infer_layer(rel_path: &str, file: &Path) -> u8 {
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

    2 // Default to Core
}

fn is_config_file(path: &str, ext: &str) -> bool {
    let config_exts = ["toml", "yaml", "yml", "json"];
    let config_names = ["config", "settings", "cargo", "package", "tsconfig"];
    
    config_exts.contains(&ext) || config_names.iter().any(|n| path.contains(n))
}

fn create_entry(rel_path: &str, file: &Path) -> FileEntry {
    let what = infer_what(rel_path, file);
    let why = infer_why(rel_path);

    let mut entry = FileEntry::new(rel_path.to_string(), what, why);
    entry.exports = extract_exports(file);

    entry
}

fn infer_what(rel_path: &str, file: &Path) -> String {
    let stem = file.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");

    let ext = file.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" if rel_path.contains("main") => format!("Application entry point."),
        "rs" if rel_path.contains("lib") => format!("Library root and public exports."),
        "rs" if rel_path.contains("mod") => format!("Module definitions for {stem}."),
        "rs" => format!("Implements {stem} functionality."),
        "toml" if stem == "Cargo" => "Rust package manifest and dependencies.".into(),
        "json" if stem == "package" => "Node.js package manifest.".into(),
        "json" | "yaml" | "yml" => format!("Configuration for {stem}."),
        _ => format!("Handles {stem}."),
    }
}

fn infer_why(rel_path: &str) -> String {
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

fn extract_exports(file: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(file).ok()?;
    let mut exports = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("pub struct ") {
            if let Some(name) = extract_ident(trimmed, "pub struct ") {
                exports.push(name);
            }
        } else if trimmed.starts_with("pub fn ") {
            if let Some(name) = extract_ident(trimmed, "pub fn ") {
                exports.push(name);
            }
        } else if trimmed.starts_with("pub trait ") {
            if let Some(name) = extract_ident(trimmed, "pub trait ") {
                exports.push(name);
            }
        } else if trimmed.starts_with("pub enum ") {
            if let Some(name) = extract_ident(trimmed, "pub enum ") {
                exports.push(name);
            }
        }
    }

    if exports.is_empty() { None } else { Some(exports) }
}

fn extract_ident(line: &str, prefix: &str) -> Option<String> {
    let rest = line.strip_prefix(prefix)?;
    let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_')?;
    Some(rest[..end].to_string())
}

fn build_layers(classified: HashMap<u8, Vec<FileEntry>>, _root: &Path) -> Vec<Layer> {
    let names = ["Config", "Core", "Domain", "Utilities", "Tests"];
    let mut layers = Vec::new();

    for num in 0..5u8 {
        if let Some(entries) = classified.get(&num) {
            if !entries.is_empty() {
                let name = names.get(num as usize).unwrap_or(&"Other");
                let mut layer = Layer::new(num, name.to_string());
                layer.entries = entries.clone();
                layers.push(layer);
            }
        }
    }

    layers
}

fn default_legend() -> Vec<LegendEntry> {
    vec![
        LegendEntry { tag: "ENTRY".into(), definition: "Application entry point".into() },
        LegendEntry { tag: "CORE".into(), definition: "Core business logic".into() },
        LegendEntry { tag: "TYPE".into(), definition: "Data structures and types".into() },
        LegendEntry { tag: "UTIL".into(), definition: "Utility functions".into() },
    ]
}
