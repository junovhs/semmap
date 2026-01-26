# semmap

Plain-text codebase maps for LLM context.

## Install

cargo install semmap

## Usage

# Validate existing semmap
semmap validate SEMMAP.md

# Check for drift against actual codebase
semmap drift SEMMAP.md

# Apply a patch (from LLM or manual)
semmap patch SEMMAP.md patch.semmap

# Generate skeleton from codebase (you still write descriptions)
semmap init > SEMMAP.md

## Specification

See SPEC.md
```

---

## CLI Design

### Commands

```
semmap validate <file>
    --strict              Fail on warnings
    --max-tokens <n>      Override per-entry token limit (default: 200)

semmap drift <file>
    Compare semmap entries against actual filesystem.
    Reports: missing files, undocumented files, stale paths.

semmap patch <file> <patch>
    Apply structured patch. Validates before writing.
    --dry-run             Show what would change
    --backup              Write .bak before modifying

semmap init
    Walk codebase, emit skeleton with paths + token counts.
    Descriptions are empty — human/LLM fills them in.
    --include <glob>      Only include matching paths
    --exclude <glob>      Skip matching paths
```

### Validation Rules

```rust
pub struct ValidationConfig {
    /// Max tokens per file entry description
    pub max_tokens_per_entry: usize,  // default: 200
    
    /// Allowed tags (empty = any)
    pub allowed_tags: HashSet<String>,
    
    /// Require all layers to have at least one entry
    pub require_all_layers: bool,  // default: false
    
    /// All referenced paths must exist on disk
    pub check_paths_exist: bool,  // default: true
    
    /// All `→ Depends on:` targets must be documented
    pub check_dependency_coverage: bool,  // default: true
}
```

### Validation Checks

| Check | Severity | Description |
|-------|----------|-------------|
| `path_exists` | Error | Referenced file doesn't exist |
| `path_undocumented` | Warning | File exists but not in semmap |
| `unknown_tag` | Error | Tag not in allowed set |
| `token_overflow` | Error | Entry exceeds token limit |
| `duplicate_path` | Error | Same path documented twice |
| `orphan_dependency` | Warning | `→ Depends on:` references undocumented file |
| `layer_empty` | Warning | A layer section has no entries |
| `malformed_entry` | Error | Entry doesn't match expected format |

---

## Patch Format

LLMs output patches, not full rewrites. Patches are auditable and reversible.

```
# patch.semmap

ADD src/vulkan/import.rs
layer: 2
tags: CRITICAL, UNSAFE
description: DMA-BUF import logic. Wraps external memory into Vulkan images for zero-copy video frame handoff.
depends_on: device.rs, surface.rs
exports: ImportedImage, import_dmabuf

UPDATE src/vulkan/device.rs
tags: CRITICAL
description: Logical device creation. Now also initializes import extensions when available.

REMOVE src/vulkan/legacy.rs

MOVE src/old/path.rs -> src/new/path.rs
```

### Patch Rules

1. `ADD` fails if path already documented
2. `UPDATE` fails if path not documented
3. `REMOVE` fails if path not documented
4. `MOVE` updates path, preserves description unless also updating
5. All patches validated before any are applied
6. Atomic: all succeed or none applied

---
