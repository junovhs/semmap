# SEMMAP CLI — Handover Document

## What We Built

A Rust CLI tool for managing Semantic Map (SEMMAP) documentation files. SEMMAPs are structured markdown files that map codebases for LLM consumption, prioritizing semantic intent over file structure.

### Commands

| Command | Purpose |
|---------|---------|
| `semmap generate` | Scan codebase → produce SEMMAP.md with auto-inferred layers |
| `semmap validate` | Check SEMMAP format + verify referenced files exist |
| `semmap deps` | Analyze imports → Mermaid diagram + layer violation checks |
| `semmap update` | Sync SEMMAP with codebase changes (add new, remove deleted) |

### Installation

```bash
cd semmap
cargo install --path .
```

## Architecture

```
src/
├── main.rs        # CLI definition (clap)
├── commands.rs    # Command implementations
├── lib.rs         # Public exports
├── parser.rs      # Parse SEMMAP markdown → SemmapFile struct
├── generator.rs   # Scan codebase → SemmapFile struct
├── validator.rs   # Check SemmapFile for issues
├── formatter.rs   # SemmapFile → markdown/json/toml output
├── deps.rs        # Dependency analysis + mermaid rendering
├── types.rs       # Core data structures
└── error.rs       # Error types
```

## Code Quality

This project is enforced by **SlopChop** with strict rules:
- No `.unwrap()` or `.expect()` — use `?`, `if let`, `map_or`, etc.
- No direct indexing (`arr[i]`) — use `.get(i)`
- `clippy::pedantic` enabled
- File token limit: 2000
- Cognitive complexity limit: 25

All clippy lints pass as of handover.

## Known Bug: `update` Command Path Mismatch

### Symptom
Running `semmap update` with `--root ./crates` wipes all entries:
```
✓ Updated SEMMAP: +0 -40
  - crates/nitrate-pal/src/...  (removes everything)
```

### Cause
Path prefix mismatch between SEMMAP entries and generator output:
- SEMMAP contains: `crates/nitrate-pal/src/lib.rs`
- Generator produces: `nitrate-pal/src/lib.rs` (relative to `--root`)

The set difference sees zero overlap → thinks all files were "removed."

### Location
`src/commands.rs`, function `update()`, around line 128.

### Fix Needed
When comparing paths, either:
1. Strip the root prefix from SEMMAP paths before comparison, OR
2. Prepend the root prefix to generated paths before comparison

Something like:
```rust
// Option 1: Normalize SEMMAP paths
let root_str = root.to_string_lossy();
let existing: HashSet<String> = semmap.all_paths()
    .into_iter()
    .map(|p| p.strip_prefix(&format!("{}/", root_str)).unwrap_or(p).to_string())
    .collect();

// Option 2: Prefix generated paths
let current: HashSet<String> = fresh.all_paths()
    .into_iter()
    .map(|p| format!("{}/{}", root_str, p))
    .collect();
```

Test with:
```bash
touch crates/nitrate-core/src/utils.rs
semmap update --root ./crates
# Should show: +1 -0, adding utils.rs
# Should NOT remove existing entries
git diff SEMMAP.md
rm crates/nitrate-core/src/utils.rs
semmap update --root ./crates
```

## SEMMAP Format Reference

```markdown
# project-name — Semantic Map

**Purpose:** One sentence describing the project goal.

## Legend

`[TAG]` Definition of the tag.

## Layer 0 — Config

`path/to/file.ext`
[What it does]. [Why it exists].
→ Exports: comma, separated, list
→ Touch: warnings, gotchas, dependencies

## Layer 1 — Core

...
```

### Rules
- Paths must be backtick-wrapped
- Description = WHAT sentence + WHY sentence (period-separated)
- `→ Exports:` and `→ Touch:` are optional metadata
- Tags like `[CRITICAL]` go in the description, not the path line
- Layers numbered 0-N, typically: Config → Core → Platform → Domain → App → Docs

## Files to Exclude from SlopChop

Large documentation files that exceed token limits should go in `.slopchopignore`:
```
research.md
example.md
docs/
```

Or move them to a `docs/` folder and ignore that.

## Next Steps

1. **Fix the `update` path bug** (see above)
2. **Add `--root` awareness to validate** — currently validates paths relative to CWD, not `--root`
3. **Improve layer inference** — current heuristics are basic (looks for "main", "test", "util" in path)
4. **Add `semmap diff`** — show what changed between SEMMAP and codebase without modifying
5. **Add `semmap check`** — combine validate + deps --check in one command
6. **Tests** — currently zero test coverage

## Handoff Protocol

When passing work to the next AI:

1. **Start with this document** — paste or attach `handover.md`
2. **Include the bug report** — if there's a specific issue, paste the slopchop/clippy output
3. **Provide file context** — the AI can request specific files with "show me src/commands.rs"
4. **Request a zip** — when done, ask for "ONE ZIP FILE" to avoid copy-paste errors
5. **Test before accepting** — run `slopchop check` or at minimum `cargo clippy` and `semmap validate`

### Template prompt for next session:

```
Here's the handover doc for the semmap project: [paste handover.md]

Current issue: [describe problem]

Here's the error output: [paste error]

Please fix this and give me a zip file with the updated src/ folder.
```
