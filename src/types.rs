use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemmapFile {
    pub project_name: String,
    pub purpose: String,
    pub legend: Vec<LegendEntry>,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendEntry {
    pub tag: String,
    pub definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub number: u8,
    pub name: String,
    pub entries: Vec<FileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub description: Description,
    pub exports: Option<Vec<String>>,
    pub touch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub what: String,
    pub why: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyMap {
    pub nodes: Vec<DepNode>,
    pub edges: Vec<DepEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepNode {
    pub path: String,
    pub layer: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepEdge {
    pub from: String,
    pub to: String,
    pub kind: DepKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DepKind {
    Import,
    Trait,
    Call,
}

impl SemmapFile {
    pub fn new(project_name: String, purpose: String) -> Self {
        Self {
            project_name,
            purpose,
            legend: Vec::new(),
            layers: Vec::new(),
        }
    }

    pub fn all_paths(&self) -> Vec<&str> {
        self.layers
            .iter()
            .flat_map(|l| l.entries.iter().map(|e| e.path.as_str()))
            .collect()
    }

    pub fn find_entry(&self, path: &str) -> Option<&FileEntry> {
        self.layers
            .iter()
            .flat_map(|l| &l.entries)
            .find(|e| e.path == path)
    }

    pub fn path_to_layer(&self) -> HashMap<&str, u8> {
        let mut map = HashMap::new();
        for layer in &self.layers {
            for entry in &layer.entries {
                map.insert(entry.path.as_str(), layer.number);
            }
        }
        map
    }
}

impl Layer {
    pub fn new(number: u8, name: String) -> Self {
        Self {
            number,
            name,
            entries: Vec::new(),
        }
    }
}

impl FileEntry {
    pub fn new(path: String, what: String, why: String) -> Self {
        Self {
            path,
            description: Description { what, why },
            exports: None,
            touch: None,
        }
    }
}

impl DependencyMap {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl Default for DependencyMap {
    fn default() -> Self {
        Self::new()
    }
}
