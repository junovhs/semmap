# SEMMAP CLI — Handover Document

## What We Built

A Rust CLI tool for managing Semantic Map (SEMMAP) documentation files. SEMMAPs are structured markdown files that map codebases for LLM consumption, prioritizing semantic intent over file structure.

### Commands

| Command | Purpose |
|---------|---------|
| semmap generate | Scan codebase → produce SEMMAP.md with auto-inferred layers |
| semmap validate | Check SEMMAP format + verify referenced files exist |
| semmap deps | Analyze imports → Mermaid diagram + layer violation checks |
| semmap update | Sync SEMMAP with codebase changes (add new, remove deleted) |

### Installation

    cd semmap
    cargo install --path .

## Architecture

    src/
    ├── main.rs          # CLI definition (clap)
    ├── commands.rs      # Command implementations
    ├── lib.rs           # Public exports
    ├── parser.rs        # Parse SEMMAP markdown → SemmapFile struct
    ├── parse_entries.rs # File entry parsing (split from parser.rs)
    ├── generator.rs     # Scan codebase → SemmapFile struct
    ├── inference.rs     # Layer/what/why inference (split from generator.rs)
    ├── exports.rs       # Rust export extraction (split from generator.rs)
    ├── validator.rs     # Check SemmapFile for issues
    ├── formatter.rs     # SemmapFile → markdown/json/toml output
    ├── deps.rs          # Dependency analysis + mermaid rendering
    ├── lang_python.rs   # Python import extraction (split from deps.rs)
    ├── path_utils.rs    # Path prefix normalization helpers
    ├── types.rs         # Core data structures
    └── error.rs         # Error types

## Code Quality

This project is enforced by SlopChop with strict rules:
- No .unwrap() or .expect() — use ?, if let, map_or, etc.
- No direct indexing (arr[i]) — use .get(i)
- clippy::pedantic enabled
- File token limit: 1500
- Cognitive complexity limit: 15

All clippy lints pass. All SlopChop checks pass.

## Session 2 Work (January 2026)

### Bug Fixed: update Command Path Mismatch

Symptom: Running semmap update with --root ./crates wiped all entries.

Cause: Path prefix mismatch between SEMMAP entries and generator output:
- SEMMAP contained: crates/nitrate-pal/src/lib.rs
- Generator produced: nitrate-pal/src/lib.rs (relative to --root)

Fix: Created src/path_utils.rs with three functions:
- build_root_prefix() — normalizes root path (strips ./, handles .)
- prefix_path() — prepends root prefix to generated paths
- strip_prefix_for_lookup() — strips prefix when looking up in fresh SemmapFile

### Refactor: SlopChop Compliance

Several files exceeded the 1500 token limit and 15 cognitive complexity limit. We split them:

| Original File | Extracted To | Reason |
|---------------|--------------|--------|
| generator.rs (1878 tokens) | exports.rs, inference.rs | Token limit + extract_exports complexity 19 |
| parser.rs (1711 tokens) | parse_entries.rs | Token limit + parse_header complexity 22 |
| deps.rs (1665 tokens) | lang_python.rs | Token limit + extract_python_imports complexity 16 |
| commands.rs (1613 tokens) | path_utils.rs | Token limit |

Final metrics:
- 21 files analyzed
- 14.5K total tokens
- 690 avg tokens/file
- 114 total complexity
- 5 avg complexity
- All checks pass

## Known Issues / Quality Concerns

### Generated Output is Generic

The current semmap generate output is functional but not insightful:

    formatter.rs
    Implements formatter functionality. Separates concerns for maintainability.

This describes WHAT but the WHY is always the same boilerplate. The inference heuristics in src/inference.rs are basic — they look for keywords like "main", "test", "util" in paths.

Potential improvements:
- Parse doc comments (/// or //!) for better descriptions
- Analyze actual dependencies to infer architectural role
- Use file content patterns (e.g., "has HTTP handlers" → "API layer")

### Exports List Has Duplicates

The exports extraction shows new multiple times:

    → Exports: SemmapFile, ..., new, all_paths, ..., new, new, new

This is because multiple impl blocks define new(). Should deduplicate or qualify with type name.

## Next Steps: Integration Tests

Priority: Write integration tests that would survive mutation testing.

### Test Categories Needed

1. Parser Tests (parser.rs, parse_entries.rs)
   - Parse valid SEMMAP → correct SemmapFile struct
   - Parse with missing title → ParseError
   - Parse with multiple layers → correct layer ordering
   - Parse exports/touch metadata → correctly attached to FileEntry
   - Parse description → splits into what/why correctly

2. Generator Tests (generator.rs, inference.rs, exports.rs)
   - Generate from empty dir → empty layers
   - Generate from dir with .rs files → correct layer assignment
   - Generate with --name/--purpose → overrides defaults
   - Config files (Cargo.toml) → Layer 0
   - Test files → Layer 4
   - Export extraction → finds pub struct/fn/trait/enum

3. Validator Tests (validator.rs)
   - Valid SEMMAP → is_valid() true
   - Missing project name → error
   - Missing purpose → warning (not error)
   - Duplicate paths → error
   - File not found → error
   - Layer gap → warning

4. Update Command Tests (commands.rs, path_utils.rs)
   - Update with new file → adds to SEMMAP
   - Update with deleted file → removes from SEMMAP
   - Update with --root prefix → paths match correctly (the bug we fixed)
   - Update preserves existing descriptions

5. Deps Tests (deps.rs, lang_python.rs)
   - Rust use crate::foo → edge to foo.rs
   - Rust mod bar; → edge to bar.rs
   - Python from .foo import → edge to foo.py
   - Layer violation detection → lower depending on higher flagged
   - Mermaid output → valid graph syntax

### Mutation Testing Criteria

Tests should be written such that:
- Changing > to >= in comparisons breaks a test
- Removing an if branch breaks a test
- Swapping error/warning severity breaks a test
- Off-by-one in layer numbers breaks a test

Use cargo mutagen or similar to verify test robustness.

## Handoff Protocol

When passing work to the next AI:

1. Start with this document — paste or attach HANDOVER.md
2. Include the bug report — if there's a specific issue, paste the slopchop/clippy output
3. Provide file context — the AI can request specific files with "show me src/commands.rs"
4. Request focused changes — small, atomic changes work better than large rewrites
5. Test before accepting — run slopchop check before considering work complete

### Template prompt for next session:

    Here's the handover doc for the semmap project: [paste HANDOVER.md]

    Current task: Write integration tests for [specific module]

    Please write tests that would survive mutation testing.

## SEMMAP Format Reference

    # project-name — Semantic Map

    **Purpose:** One sentence describing the project goal.

    ## Legend

    [TAG] Definition of the tag.

    ## Layer 0 — Config

    path/to/file.ext
    [What it does]. [Why it exists].
    → Exports: comma, separated, list
    → Touch: warnings, gotchas, dependencies

    ## Layer 1 — Core

    ...

### Rules
- Paths must be backtick-wrapped in actual SEMMAP files
- Description = WHAT sentence + WHY sentence (period-separated)
- → Exports: and → Touch: are optional metadata
- Tags like [CRITICAL] go in the description, not the path line
- Layers numbered 0-N, typically: Config → Core → Platform → Domain → App → Docs




***UPDATE - THIS IS THE LATEST INFORMATION***

# Antigravity Handoff — January 2026 1:39PM

## Work Completed This Session

### 1. Integration Testing Framework
- Added `tempfile` as a dev-dependency in `Cargo.toml` to support robust filesystem-based testing.
- Created `tests/parser_tests.rs` covering valid/invalid parsing, layers, and metadata extraction.
- Created `tests/generator_tests.rs` covering layer inference, export extraction, and exclusion rules.

### 2. Bug Fixes
- **Parser (`src/parser.rs`):** Updated regex patterns to correctly handle dashes (`--`) and single dashes in layer/title headers.
- **Entry Parsing (`src/parse_entries.rs`):** Fixed logic where `strip_prefix` failed because leading spaces were already removed by `trim()`.
- **Generator (`src/generator.rs`):** Fixed a major bug in `is_excluded` where the root directory (depth 0) was being filtered out if it started with a dot (common with `tempfile` paths), causing empty maps.

## Current Project State

### SlopChop Status
- **Parser Tests:** Green and merged to `main`.
- **Generator Tests:** Blue/Red. They are **functionally passing** (all 7 tests pass), but they violate SlopChop constraints:
    - `LAW OF ATOMICITY`: `tests/generator_tests.rs` is ~1610 tokens (Limit: 1500).
    - `CLIPPY`: Contains several `clippy::unwrap_used` and `clippy::doc-markdown` violations that need refactoring to `Result` returns.

### Files Modified
- `Cargo.toml` (added `tempfile`)
- `src/parser.rs` (regex fixes)
- `src/parse_entries.rs` (logic fix)
- `src/generator.rs` (WalkDir fix)
- `tests/parser_tests.rs` (NEW)
- `tests/generator_tests.rs` (NEW - needs refactoring for SlopChop)

## Instructions for Next Session
1. **Refactor `tests/generator_tests.rs`**: Split the file or remove the `debug_walkdir` test to bring it under 1500 tokens. 
2. **Clean up Clippy**: Convert tests to return `TestResult` and use `?` instead of `unwrap()`.
3. **Continue Integration Tests**:
    - `validator.rs`
    - `commands.rs` (Update flow)
    - `deps.rs` (Dependency analysis)
