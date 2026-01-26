use crate::types::{DepEdge, DepKind, DepNode, DependencyMap, SemmapFile};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn analyze(root: &Path, semmap: &SemmapFile) -> DependencyMap {
    let mut depmap = DependencyMap::new();
    let path_to_layer = semmap.path_to_layer();

    for path in semmap.all_paths() {
        let layer = path_to_layer.get(path).copied().unwrap_or(0);
        depmap.nodes.push(DepNode { path: path.to_string(), layer });
    }

    let known_paths: HashSet<_> = semmap.all_paths().into_iter().collect();
    
    for path in semmap.all_paths() {
        let full_path = root.join(path);
        if let Ok(content) = fs::read_to_string(&full_path) {
            let imports = extract_imports(&content, path);
            for (target, kind) in imports {
                if known_paths.contains(target.as_str()) && target != path {
                    depmap.edges.push(DepEdge {
                        from: path.to_string(),
                        to: target,
                        kind,
                    });
                }
            }
        }
    }

    depmap
}

fn extract_imports(content: &str, source_path: &str) -> Vec<(String, DepKind)> {
    let ext = Path::new(source_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => extract_rust_imports(content, source_path),
        "ts" | "js" => extract_js_imports(content, source_path),
        "py" => extract_python_imports(content),
        _ => Vec::new(),
    }
}

fn extract_rust_imports(content: &str, source_path: &str) -> Vec<(String, DepKind)> {
    let mut deps = Vec::new();
    
    let use_re = Regex::new(r"use\s+(?:crate|super|self)::(\w+)").ok();
    let mod_re = Regex::new(r"mod\s+(\w+);").ok();

    let base_dir = Path::new(source_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    if let Some(re) = use_re {
        for cap in re.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let module = m.as_str();
                let target = resolve_rust_module(&base_dir, module);
                deps.push((target, DepKind::Import));
            }
        }
    }

    if let Some(re) = mod_re {
        for cap in re.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let module = m.as_str();
                let target = resolve_rust_module(&base_dir, module);
                deps.push((target, DepKind::Import));
            }
        }
    }

    deps
}

fn resolve_rust_module(base_dir: &str, module: &str) -> String {
    if base_dir.is_empty() {
        format!("src/{}.rs", module)
    } else {
        format!("{}/{}.rs", base_dir, module)
    }
}

fn extract_js_imports(content: &str, source_path: &str) -> Vec<(String, DepKind)> {
    let mut deps = Vec::new();
    
    let import_re = Regex::new(r#"(?:import|from)\s+['"]([./][^'"]+)['"]"#).ok();
    let require_re = Regex::new(r#"require\(['"]([./][^'"]+)['"]\)"#).ok();

    let base_dir = Path::new(source_path).parent();

    let extract = |re: Option<Regex>| -> Vec<String> {
        re.map(|r| {
            r.captures_iter(content)
                .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
                .collect()
        }).unwrap_or_default()
    };

    for relative in extract(import_re).into_iter().chain(extract(require_re)) {
        if let Some(resolved) = resolve_js_path(base_dir, &relative) {
            deps.push((resolved, DepKind::Import));
        }
    }

    deps
}

fn resolve_js_path(base: Option<&Path>, relative: &str) -> Option<String> {
    let base = base?;
    let mut path = base.join(relative);
    
    if !path.extension().is_some() {
        path.set_extension("ts");
        if !path.exists() {
            path.set_extension("js");
        }
    }

    Some(path.to_string_lossy().to_string())
}

fn extract_python_imports(content: &str) -> Vec<(String, DepKind)> {
    let mut deps = Vec::new();
    
    let import_re = Regex::new(r"from\s+\.(\w+)\s+import").ok();
    let simple_re = Regex::new(r"import\s+(\w+)").ok();

    if let Some(re) = import_re {
        for cap in re.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                deps.push((format!("{}.py", m.as_str()), DepKind::Import));
            }
        }
    }

    if let Some(re) = simple_re {
        for cap in re.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let module = m.as_str();
                if !is_stdlib(module) {
                    deps.push((format!("{}.py", module), DepKind::Import));
                }
            }
        }
    }

    deps
}

fn is_stdlib(module: &str) -> bool {
    let stdlib = ["os", "sys", "re", "json", "typing", "collections", "pathlib"];
    stdlib.contains(&module)
}

pub fn render_mermaid(depmap: &DependencyMap) -> String {
    let mut out = String::from("graph TD\n");

    for node in &depmap.nodes {
        let id = sanitize_id(&node.path);
        let label = Path::new(&node.path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| node.path.clone());
        out.push_str(&format!("    {}[\"{}\"]\n", id, label));
    }

    for edge in &depmap.edges {
        let from_id = sanitize_id(&edge.from);
        let to_id = sanitize_id(&edge.to);
        let arrow = match edge.kind {
            DepKind::Import => "-->",
            DepKind::Trait => "-.->",
            DepKind::Call => "==>",
        };
        out.push_str(&format!("    {} {} {}\n", from_id, arrow, to_id));
    }

    out
}

fn sanitize_id(path: &str) -> String {
    path.replace(['/', '.', '-'], "_")
}

pub fn check_layer_violations(depmap: &DependencyMap, semmap: &SemmapFile) -> Vec<String> {
    let mut violations = Vec::new();
    let path_to_layer = semmap.path_to_layer();

    for edge in &depmap.edges {
        let from_layer = path_to_layer.get(edge.from.as_str()).copied();
        let to_layer = path_to_layer.get(edge.to.as_str()).copied();

        if let (Some(fl), Some(tl)) = (from_layer, to_layer) {
            if tl > fl {
                violations.push(format!(
                    "Layer violation: {} (L{}) depends on {} (L{})",
                    edge.from, fl, edge.to, tl
                ));
            }
        }
    }

    violations
}
