use std::path::Path;

pub fn build_root_prefix(root: &Path) -> String {
    let root_str = root.to_string_lossy();
    let cleaned = root_str.trim_start_matches("./");
    if cleaned == "." || cleaned.is_empty() {
        String::new()
    } else {
        cleaned.to_string()
    }
}

pub fn prefix_path(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_string()
    } else {
        format!("{prefix}/{path}")
    }
}

pub fn strip_prefix_for_lookup(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_string()
    } else {
        path.strip_prefix(&format!("{prefix}/"))
            .unwrap_or(path)
            .to_string()
    }
}