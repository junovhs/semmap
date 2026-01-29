use crate::path_utils;
use crate::types::Layer;
use crate::SemmapFile;

pub fn add_new_entries(semmap: &mut SemmapFile, added: &[String], fresh: &SemmapFile, prefix: &str) {
    for path in added {
        let lookup = path_utils::strip_prefix_for_lookup(prefix, path);
        if let Some(entry) = fresh.find_entry(&lookup) {
            let inferred = fresh.layers.iter()
                .find(|l| l.entries.iter().any(|e| e.path == lookup))
                .map_or(2, |l| l.number);

            let idx = if let Some(pos) = semmap.layers.iter().position(|l| l.number == inferred) {
                pos
            } else {
                semmap.layers.push(Layer::new(inferred, format!("Layer {inferred}")));
                semmap.layers.sort_by_key(|l| l.number);
                semmap.layers.iter().position(|l| l.number == inferred)
                    .unwrap_or(0)
            };

            if let Some(layer) = semmap.layers.get_mut(idx) {
                let mut e = entry.clone();
                e.path.clone_from(path);
                layer.entries.push(e);
            }
        }
    }
}

pub fn remove_deleted_entries(semmap: &mut SemmapFile, removed: &[String]) {
    let removed_set: std::collections::HashSet<&str> = removed.iter().map(String::as_str).collect();
    for layer in &mut semmap.layers {
        layer.entries.retain(|e| !removed_set.contains(e.path.as_str()));
    }
    // Remove empty layers
    semmap.layers.retain(|l| !l.entries.is_empty());
}
