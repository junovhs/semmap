use std::path::Path;

/// Computes the path prefix for entries based on root's position relative to `semmap_dir`.
/// If root is the same as `semmap_dir`, returns empty string.
/// If root is a subdirectory of `semmap_dir`, returns the relative path.
pub fn build_root_prefix_relative(semmap_dir: &Path, root: &Path) -> String {
    // Try to strip semmap_dir from root to get relative path
    if let Ok(relative) = root.strip_prefix(semmap_dir) {
        let rel_str = relative.to_string_lossy();
        let cleaned = rel_str
            .trim_start_matches("./")
            .trim_start_matches(".\\");
        if cleaned == "." || cleaned.is_empty() {
            return String::new();
        }
        // Normalize path separators to forward slashes
        return cleaned.replace('\\', "/");
    }

    // If root is not under semmap_dir, fall back to original behavior
    build_root_prefix(root)
}

pub fn build_root_prefix(root: &Path) -> String {
    let root_str = root.to_string_lossy();
    let cleaned = root_str
        .trim_start_matches("./")
        .trim_start_matches(".\\");
    if cleaned == "." || cleaned.is_empty() {
        String::new()
    } else {
        // Normalize path separators
        cleaned.replace('\\', "/")
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
