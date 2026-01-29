use crate::path_utils;
use crate::types::Layer;
use crate::SemmapFile;
use std::collections::HashMap;

pub fn add_new_entries(
    semmap: &mut SemmapFile,
    added: &[String],
    fresh: &SemmapFile,
    prefix: &str,
) {
    // Pre-build lookup maps to avoid linear search in loop
    let path_to_layer: HashMap<&str, u8> = fresh.path_to_layer();
    let entry_map: HashMap<&str, &crate::types::FileEntry> = fresh
        .layers
        .iter()
        .flat_map(|l| l.entries.iter().map(|e| (e.path.as_str(), e)))
        .collect();

    // Group new entries by their target layer
    let mut by_layer: HashMap<u8, Vec<crate::types::FileEntry>> = HashMap::new();
    for path in added {
        let lookup = path_utils::strip_prefix_for_lookup(prefix, path);
        let Some(entry) = entry_map.get(lookup.as_str()) else {
            continue;
        };
        let layer_num = path_to_layer.get(lookup.as_str()).copied().unwrap_or(2);
        let mut e = (*entry).clone();
        e.path.clone_from(path);
        by_layer.entry(layer_num).or_default().push(e);
    }

    // Build layer number to index map
    let layer_idx: HashMap<u8, usize> = semmap
        .layers
        .iter()
        .enumerate()
        .map(|(i, l)| (l.number, i))
        .collect();

    // Insert entries into existing or new layers
    for (num, entries) in by_layer {
        if let Some(&idx) = layer_idx.get(&num) {
            if let Some(l) = semmap.layers.get_mut(idx) {
                l.entries.extend(entries);
            }
        } else {
            let mut layer = Layer::new(num, format!("Layer {num}"));
            layer.entries = entries;
            semmap.layers.push(layer);
        }
    }

    semmap.layers.sort_by_key(|l| l.number);
    for layer in &mut semmap.layers {
        layer.entries.sort_by(|a, b| a.path.cmp(&b.path));
    }
}

pub fn remove_deleted_entries(semmap: &mut SemmapFile, removed: &[String]) {
    let removed_set: std::collections::HashSet<&str> = removed.iter().map(String::as_str).collect();
    for layer in &mut semmap.layers {
        layer
            .entries
            .retain(|e| !removed_set.contains(e.path.as_str()));
    }
    semmap.layers.retain(|l| !l.entries.is_empty());
}
