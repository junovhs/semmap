# SEMMAP Generate Improvement Proposal

## Problem Statement

Current `semmap generate` output is generic:

```
formatter.rs
Implements formatter functionality. Separates concerns for maintainability.
```

The WHAT is vague ("implements functionality") and the WHY is boilerplate.

## Research Synthesis

Based on SOTA analysis, we can improve descriptions using **four programmatic techniques** that don't require LLMs at runtime:

| Technique | Source of Truth | Provides |
|-----------|-----------------|----------|
| Doc Comment Extraction | `///` and `//!` comments | Developer-written WHAT |
| SWUM (Identifier Expansion) | Function/struct names | Verb-noun WHAT sentences |
| Stereotype Classification | Code patterns + imports | Architectural role (WHY) |
| Fan-in/Fan-out Analysis | Dependency graph | Importance/layer hints |

## Proposed Implementation

### Phase 1: Doc Comment Extraction (High Value, Low Effort)

**Goal:** Use existing developer documentation as the primary description source.

**Implementation:**
```rust
// src/doc_extractor.rs

/// Extract the first doc comment block from a Rust file.
/// Returns the module-level `//!` comment if present, else the first `///` block.
pub fn extract_doc_comment(content: &str) -> Option<String> {
    // Priority 1: Module-level //! comments
    let module_doc = extract_module_doc(content);
    if module_doc.is_some() {
        return module_doc;
    }
    
    // Priority 2: First /// comment block (usually on main struct/fn)
    extract_first_item_doc(content)
}
```

**Example Output:**
```
// Input: src/formatter.rs with //! Converts SemmapFile to markdown/json/toml
formatter.rs
Converts SemmapFile to markdown/json/toml. Supports multiple output formats.
```

**Fallback:** If no doc comment exists, fall back to SWUM.

---

### Phase 2: SWUM Identifier Expansion (Medium Value, Medium Effort)

**Goal:** Convert function names to readable sentences when no doc comments exist.

**Algorithm:**
1. Split identifier: `get_user_profile` → `["get", "user", "profile"]`
2. Tag parts-of-speech: `get` = verb, `user profile` = noun phrase
3. Construct sentence: "Gets the user profile"

**Implementation:**
```rust
// src/swum.rs

pub fn expand_identifier(name: &str) -> String {
    let words = split_identifier(name); // camelCase and snake_case
    
    if words.is_empty() {
        return format!("Implements {} functionality.", name);
    }
    
    let first = words.first().map(String::as_str).unwrap_or("");
    let rest = words.get(1..).unwrap_or(&[]).join(" ");
    
    // Verb-first pattern (get_*, set_*, is_*, has_*, create_*, etc.)
    match first {
        "get" | "fetch" | "load" | "read" => format!("Gets the {}.", rest),
        "set" | "update" | "write" => format!("Sets the {}.", rest),
        "is" | "has" | "can" | "should" => format!("Checks if {}.", rest),
        "create" | "new" | "build" | "make" => format!("Creates a new {}.", rest),
        "delete" | "remove" | "drop" => format!("Removes the {}.", rest),
        "parse" | "extract" => format!("Parses {}.", rest),
        "validate" | "check" => format!("Validates {}.", rest),
        "render" | "format" | "display" => format!("Renders {} output.", rest),
        "handle" | "process" => format!("Handles {}.", rest),
        _ => format!("Implements {} {}.", first, rest),
    }
}

fn split_identifier(name: &str) -> Vec<String> {
    // Handle snake_case
    if name.contains('_') {
        return name.split('_').map(|s| s.to_lowercase()).collect();
    }
    // Handle camelCase/PascalCase
    split_camel_case(name)
}
```

**Example:**
```
// File: parse_entries.rs (no doc comment)
// Main function: parse_entry_block

parse_entries.rs
Parses entry block. Extracts file entries from SEMMAP layers.
```

---

### Phase 3: Stereotype Classification (High Value, High Effort)

**Goal:** Infer architectural role from code patterns to generate the WHY.

**Stereotypes to Detect:**

| Stereotype | Detection Signal | Generated WHY |
|------------|------------------|---------------|
| **Config** | Filename: `*.toml`, `*.json`, `config.*` | "Defines project configuration." |
| **Entity/Model** | Mostly `pub struct` + getters/setters | "Represents domain data." |
| **Service** | High cyclomatic complexity, calls other modules | "Orchestrates business logic." |
| **Repository** | Imports `sql`, `diesel`, `sqlx`, file I/O | "Handles data persistence." |
| **Handler** | Imports `axum`, `actix`, `rocket`, HTTP annotations | "Handles HTTP requests." |
| **Utility** | All `pub fn`, no state, high fan-in | "Provides reusable helpers." |
| **Parser** | Regex imports, returns structured data | "Parses input into structured data." |
| **Formatter** | Takes struct, returns String | "Formats data for output." |
| **Error** | Defines `Error` enum or `impl Error` | "Defines error types for the crate." |
| **CLI** | Imports `clap`, `structopt`, `argh` | "Defines command-line interface." |
| **Test** | `#[test]` or `#[cfg(test)]` | "Tests module functionality." |

**Implementation:**
```rust
// src/stereotype.rs

pub enum Stereotype {
    Config,
    Entity,
    Service,
    Repository,
    Handler,
    Utility,
    Parser,
    Formatter,
    Error,
    Cli,
    Test,
    Unknown,
}

pub fn classify(path: &str, content: &str, fan_in: usize, fan_out: usize) -> Stereotype {
    // 1. Filename heuristics (fast path)
    if is_config_file(path) { return Stereotype::Config; }
    if is_test_file(path) { return Stereotype::Test; }
    
    // 2. Import-based detection
    if has_http_framework_imports(content) { return Stereotype::Handler; }
    if has_db_imports(content) { return Stereotype::Repository; }
    if has_cli_imports(content) { return Stereotype::Cli; }
    
    // 3. Code pattern detection
    if is_mostly_structs_and_accessors(content) { return Stereotype::Entity; }
    if is_error_module(content) { return Stereotype::Error; }
    
    // 4. Fan-in/out heuristics
    if fan_in > 5 && fan_out < 2 { return Stereotype::Utility; }
    if fan_out > 5 { return Stereotype::Service; }
    
    Stereotype::Unknown
}

pub fn stereotype_to_why(s: Stereotype) -> &'static str {
    match s {
        Stereotype::Config => "Defines project configuration.",
        Stereotype::Entity => "Represents domain data structures.",
        Stereotype::Service => "Orchestrates business logic between modules.",
        Stereotype::Repository => "Handles data persistence and retrieval.",
        Stereotype::Handler => "Handles incoming HTTP requests.",
        Stereotype::Utility => "Provides reusable helper functions.",
        Stereotype::Parser => "Parses input into structured data.",
        Stereotype::Formatter => "Formats data for output or display.",
        Stereotype::Error => "Defines error types for the crate.",
        Stereotype::Cli => "Defines the command-line interface.",
        Stereotype::Test => "Verifies module correctness.",
        Stereotype::Unknown => "Supports application functionality.",
    }
}
```

---

### Phase 4: Fan-in/Fan-out Integration (Low Effort, Uses Existing Code)

**Goal:** Use existing `deps.rs` analysis to inform stereotype classification.

We already have `deps::analyze()` which builds a dependency graph. We can compute:

```rust
// Add to deps.rs or inference.rs

pub struct ModuleMetrics {
    pub fan_in: usize,   // How many files import this one
    pub fan_out: usize,  // How many files this one imports
}

pub fn compute_metrics(depmap: &DependencyMap, path: &str) -> ModuleMetrics {
    let fan_in = depmap.edges.iter().filter(|e| e.to == path).count();
    let fan_out = depmap.edges.iter().filter(|e| e.from == path).count();
    ModuleMetrics { fan_in, fan_out }
}
```

**Interpretation:**
- High fan-in + low fan-out → Utility/Core (stable, everyone uses it)
- Low fan-in + high fan-out → Orchestrator/Entry point (volatile, uses everything)
- Balanced → Domain logic

---

## Proposed Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                        generate(root)                           │
└─────────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│ 1. Walk files, build dependency graph (existing)                │
└─────────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. For each file:                                               │
│    a. Extract doc comment (//! or ///)                          │
│    b. If none, use SWUM on primary identifier                   │
│    c. Classify stereotype from imports + patterns + metrics     │
│    d. Generate WHAT from (a) or (b)                             │
│    e. Generate WHY from stereotype                              │
└─────────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. Assign layer from stereotype (existing inference.rs)         │
└─────────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. Output SEMMAP.md                                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Expected Output Improvement

### Before:
```markdown
## Layer 2 — Domain

`src/formatter.rs`
Implements formatter functionality. Separates concerns for maintainability.

`src/parser.rs`
Implements parser functionality. Separates concerns for maintainability.
```

### After:
```markdown
## Layer 2 — Domain

`src/formatter.rs`
Converts SemmapFile to markdown, JSON, or TOML output. Formats data for output or display.
  → Exports: to_markdown, to_json, to_toml

`src/parser.rs`
Parses SEMMAP markdown into a SemmapFile struct. Parses input into structured data.
  → Exports: parse
```

---

## Implementation Plan

| Phase | New Files | Tokens Est. | Effort |
|-------|-----------|-------------|--------|
| 1. Doc extraction | `src/doc_extractor.rs` | ~400 | 1 hour |
| 2. SWUM | `src/swum.rs` | ~500 | 2 hours |
| 3. Stereotype | `src/stereotype.rs` | ~600 | 3 hours |
| 4. Integration | Modify `inference.rs` | ~200 | 1 hour |
| **Total** | 3 new files | ~1700 | 7 hours |

**Risk:** Each new file must stay under 1500 tokens for SlopChop compliance.

---

## Open Questions

1. **Should we parse ALL doc comments or just the first?**
   - Proposal: First `//!` (module) or first `///` (main item)

2. **How do we handle files with multiple public items?**
   - Proposal: Summarize the "primary" export (largest struct/fn)

3. **Should stereotype override doc comments for WHY?**
   - Proposal: Yes, stereotype provides consistent architectural framing

4. **Do we need language support beyond Rust?**
   - Current: Rust-only. Future: TypeScript, Python would need separate extractors.

---

## Next Steps

1. **Approve this proposal** (or request changes)
2. **Implement Phase 1** (doc extraction) — immediate value, low risk
3. **Add tests** for doc extraction
4. **Iterate** on Phases 2-4 based on real output quality



***

CHATGPT PRO ASSESMENT 

## Docs folder scan (first)

### `docs/HANDOVER.md` — operational context + constraints

* **What this repo is:** a Rust CLI for generating/validating/updating “SEMMAP” Markdown files (a semantic map of a codebase for LLM consumption). 
* **Commands shipped:** `generate`, `validate`, `deps`, `update`. 
* **Architecture snapshot:** `src/main.rs` (clap CLI) → `src/commands.rs` → modules like `parser`, `generator`, `deps`, `validator`, `formatter`, etc. 
* **Quality gates (“SlopChop” discipline):** no `unwrap`/`expect`, no direct indexing, strict clippy, max ~1500 tokens/file, cognitive complexity cap ~15. 
* **Noted historical issues:**

  * “Generate output is generic” (WHAT/WHY boilerplate). 
  * “Exports list has duplicates” (multiple `new()` in impl blocks, etc.). 
  * A prior **`update` path-prefix mismatch** bug was fixed by introducing `path_utils`. 
* **Status note:** `HANDOVER.md` includes an “UPDATE” section that mentions generator tests once violated token/clippy rules, but the later `slopchop-report.txt` shows **Status: PASSED** and all test suites green. 

### `docs/GENERATE_IMPROVEMENT_PROPOSAL.md` — improving `semmap generate` without runtime LLM calls

This doc proposes a concrete, staged plan to make generated descriptions less boilerplate:

1. **Doc comment extraction** (`//!` then first `///`) for the WHAT. 
2. **SWUM identifier expansion** for readable fallback WHAT sentences. 
3. **Stereotype classification** (config/entity/service/parser/formatter/etc.) to produce the WHY. 
4. **Fan-in/fan-out integration** via existing dependency graph analysis. 

### `docs/archived/visual-structure-research.md` — background research, not implementation guidance

It’s an archived research memo on how LLMs handle visual/indented structures; key practical implication is consistent with SEMMAP’s design: prefer **flat paths and explicit structure** over ASCII trees. 

---

## Codebase familiarization (second)

### Repository shape and “how it runs”

* **Binary entry:** `src/main.rs` defines clap subcommands and dispatches into `commands::*`. 
* **Top-level orchestration:** `src/commands.rs` implements:

  * `validate(file, root, strict)` → read file → `parser::parse` → `validator::{validate|validate_against_codebase}` → print issues. 
  * `generate(root, output, name, purpose, format)` → `generator::generate` → `formatter::{to_markdown|to_json|to_toml}` → write output. 
  * `deps(file, root, format, check)` → parse semmap → `deps::analyze` → optional `check_layer_violations` → print Mermaid/JSON. 
  * `update(file, root)` → parse existing → generate “fresh” → diff paths → add/remove entries → write formatted Markdown. 

### Core domain model (`src/types.rs`)

Everything flows through a small set of serializable structs:

* `SemmapFile { project_name, purpose, legend, layers }`
* `Layer { number, name, entries }`
* `FileEntry { path, description, exports, touch }`
* `DependencyMap { nodes, edges }` with `DepKind` enum. 

### Parsing and formatting

* **Formatter (`src/formatter.rs`)** writes Markdown with:

  * header `# {project} — Semantic Map`
  * optional purpose
  * legend
  * per-layer entries
  * **metadata lines are emitted as** `→ Exports: ...` and `→ Touch: ...`. 
* **Parser (`src/parser.rs` + `src/parse_entries.rs`)**:

  * uses regexes cached in `OnceLock` for title/purpose/legend/layer. 
  * parses file entries in `parse_entries.rs`, but **currently recognizes metadata lines as `Exports:` and `Touch:` (no arrow)**. 
  * The integration tests confirm this expected input format (indented “Exports:”/“Touch:” lines). 

**Important discrepancy to be aware of:** the project’s README/spec and the sample `SEMMAP.md` use `→ Exports:` / `→ Touch:`, but the parser only understands `Exports:` / `Touch:`. That means Markdown produced by `formatter::to_markdown()` is **not perfectly round-trippable** by `parser::parse()` w.r.t. exports/touch metadata, and `update` can inadvertently “flatten” metadata into the description text. You can see both sides in the current code/tests.

### Generating a SEMMAP

* `src/generator.rs`:

  * Walks the filesystem with `WalkDir`, honoring default include extensions and excluded dirs. 
  * Builds entries by:

    * inferring layer via `inference::infer_layer(rel_path, file)`
    * inferring `what` via `inference::infer_what(...)`
    * inferring `why` via `inference::infer_why(rel_path)`
    * extracting exports via `exports::extract_exports(file)` (Rust-only heuristic). 
* `src/inference.rs` is intentionally heuristic/path-based and is the source of the “generic output” complaint in the handover. 
* `src/exports.rs` is a line-based “pub item” scanner (struct/fn/trait/enum) and will naturally produce duplicates if the same identifier appears multiple times. 

### Dependency analysis (`src/deps.rs` + `src/lang_python.rs`)

* Builds a `DependencyMap` by:

  * creating nodes for each SEMMAP path + layer lookup
  * reading each file and extracting imports based on extension:

    * Rust: `use crate::foo` and `mod foo;` via regex, resolved into `{base_dir}/foo.rs` 
    * JS/TS: regex import/require, resolves relative paths and guesses extension 
    * Python: `lang_python::extract_imports` handles `from .foo import ...` and `import foo` (excluding a small stdlib allowlist). 
* Layer violations are defined as: **a lower-numbered layer depending on a higher-numbered layer** (`to_layer > from_layer`). 

### Validator (`src/validator.rs`)

* Validates:

  * header presence + purpose warnings
  * layers numbering/gaps
  * entry “WHAT” existence + punctuation
  * duplicate paths
  * (optionally) file existence under a provided root
  * strict mode: “files in codebase not documented.” 

---

## High-signal “gotchas” I’d keep in mind going forward

1. **Parser/formatter metadata mismatch** (`→ Exports` vs `Exports`). This is the most immediate spec/implementation inconsistency.
2. **`update` layering behavior:** new entries are currently inserted into the first existing layer rather than the inferred layer from `fresh` (because `find_entry` returns only `FileEntry` without layer context). That can silently scramble layering semantics. 
3. **Dependency extraction is intentionally shallow** (e.g., Rust `crate::foo::bar` collapses to `foo`), so “deps” is best-effort visualization + coarse layer checks, not a full module resolver. 

---

FOLLOWUP

Got it — by your definition, “production” means: **safe to run in CI/CD and rely on its output as an input to other automation** (no silent drift, no lossy transforms, deterministic diffs).

By that bar, you’re **close**, but there are **two correctness footguns** and **two determinism/polish gaps** that should be fixed before you can responsibly automate this.

## Production blockers (must-fix)

### 1) Round-trip correctness: parser/formatter metadata mismatch

* Formatter emits metadata lines with an arrow prefix (`→ Exports:` / `→ Touch:`). 
* Parser only recognizes `Exports:` / `Touch:` (no arrow). 
* README + sample SEMMAP also use the arrow style, so the “spec” aligns with formatter, not parser. 

**Why this blocks automation:** `parse → update → format` is not guaranteed to be information-preserving. That is a hard “no” for CI trust.

**Fix direction (recommended):**

* Make parser accept **both** forms:

  * `Exports:` / `Touch:`
  * `→ Exports:` / `→ Touch:`
* Then decide on **one canonical output** (I’d keep the arrow because it’s in README/sample), and ensure formatter always writes canonical.

**Acceptance tests:**

* Golden SEMMAP fixtures with both styles parse identically and reformat into canonical.
* `format(parse(format(parse(X)))) == format(parse(X))` for representative fixtures.

---

### 2) Update semantics: layer placement is wrong for new entries

`update` currently adds *all* new entries to the **first** layer (`layers.first_mut()`), regardless of inferred layer. 

**Why this blocks automation:** over repeated automated runs, architecture semantics degrade (layers become meaningless), and diffs become noisy/untrustworthy.

**Fix direction (recommended):**

* When generating the “fresh” semmap during update, retain a mapping of `path → inferred_layer_number`.
* When inserting a new entry into the existing semmap, place it into:

  * the matching existing layer number if present, else
  * create the missing layer (or insert into nearest valid layer per your layer rules).
* Avoid silently changing an existing entry’s layer unless you explicitly want “re-layering” behavior (automation-friendly default is: **do not move existing entries**).

**Acceptance tests:**

* Update with N new files across multiple inferred layers: entries land in correct layers deterministically.
* Update run twice produces identical output (idempotence).

## Determinism and stability gaps (should-fix before automation)

### 3) Export lists: duplicates + ordering instability

Known issue: exports extraction can generate duplicates (e.g., multiple `new()`), which creates noisy diffs. 

**Fix direction:**

* Dedupe exports (`HashSet` or sort+dedup) and **sort** them (stable ordering).
* Consider qualifying exports in a way that reduces collisions (optional; can be later).

**Acceptance tests:**

* Given a Rust file with repeated `pub fn new`, the final export list contains a single `new`.
* Export order is consistent between runs.

---

### 4) Stable output ordering across runs

For CI automation, you need “same repo state → same output bytes”. Generator walks the filesystem and builds entries; if traversal order or insertion order varies, you’ll get diff churn. 

**Fix direction:**

* Ensure file discovery ordering is deterministic (collect → sort by normalized relative path).
* Ensure within each layer, entries are sorted by path.
* Ensure formatting is deterministic (it mostly is, but sorting is the key).

**Acceptance tests:**

* Run `generate` twice on same fixture repo → byte-identical SEMMAP.
* Run `update` twice on same repo + same SEMMAP → byte-identical.

## What “production-ready” looks like in concrete gates

If I were setting the bar for “trust in automation”, I’d require these invariants:

1. **Round-trip safety:** `format(parse(X))` is canonical and never loses metadata. 
2. **Idempotence:** `update(update(X)) == update(X)` and `generate(generate_input)` is byte-stable. 
3. **Deterministic ordering:** exports + entries sorted; no nondeterministic traversal artifacts. 
4. **Meaning-preserving update:** new entries go to correct layers; existing entries don’t get “scrambled”. 
5. **CI enforcement:** clippy/test already green and strict; keep them as required gates.  

## A practical “productionization” sequence (fastest path)

### Phase A — correctness (highest ROI)

* Fix parser to accept `→ Exports:` / `→ Touch:` in addition to current forms. 
* Add round-trip golden tests using the sample/README style. 

### Phase B — update semantics

* Refactor update flow to preserve inferred layer for new entries. 
* Add idempotence tests for update.

### Phase C — determinism and diff hygiene

* Sort discovered paths, sort entries per layer, dedupe+sort exports. 

### Phase D — (optional) adoption improvements

* Improve generate WHAT/WHY (doc comment extraction, SWUM, stereotypes) — valuable, but not a production blocker if correctness/determinism are solved. 

---

If you want the most leverage with the least code churn: **fix the metadata mismatch + update layer placement first**. Those are the two things that can make automated runs silently degrade the map over time.
