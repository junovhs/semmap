use crate::exports;
use crate::inference;
use crate::types::{FileEntry, Layer, LegendEntry, SemmapFile};
use std::collections::HashMap;
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
            .map_or_else(|| "project".into(), |n| n.to_string_lossy().to_string())
    } else {
        config.project_name
    };

    let mut semmap = SemmapFile::new(project_name, config.purpose);
    semmap.legend = default_legend();
    semmap.layers = build_layers(&classified);

    semmap
}

fn collect_files(root: &Path, config: &GeneratorConfig) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_excluded(e, &config.exclude_dirs));

    for entry in walker.filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        let matches_ext = entry.path()
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| config.include_exts.iter().any(|inc| inc == e));

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

        let layer = inference::infer_layer(&rel_path, file);
        let entry = create_entry(&rel_path, file);

        layers.entry(layer).or_default().push(entry);
    }

    layers
}

fn create_entry(rel_path: &str, file: &Path) -> FileEntry {
    let what = inference::infer_what(rel_path, file);
    let why = inference::infer_why(rel_path);

    let mut entry = FileEntry::new(rel_path.to_string(), what, why);
    entry.exports = exports::extract_exports(file);

    entry
}

fn build_layers(classified: &HashMap<u8, Vec<FileEntry>>) -> Vec<Layer> {
    let names = ["Config", "Core", "Domain", "Utilities", "Tests"];
    let mut layers = Vec::new();

    for num in 0..5u8 {
        if let Some(entries) = classified.get(&num) {
            if !entries.is_empty() {
                let name = names.get(num as usize).copied().unwrap_or("Other");
                let mut layer = Layer::new(num, (*name).to_string());
                layer.entries.clone_from(entries);
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