# SEMMAP Language Parity Plan

**Goal:** Bring TypeScript/JavaScript, Python, Go, and Swift to feature parity with Rust.

---

## Current State

| Feature | Rust | TS/JS | Python | Go | Swift |
|---------|:----:|:-----:|:------:|:--:|:-----:|
| File discovery | ✅ | ✅ | ✅ | ✅ | ❌ |
| Layer inference | ✅ | ⚠️ | ⚠️ | ⚠️ | ❌ |
| Export extraction | ✅ | ❌ | ❌ | ❌ | ❌ |
| Doc comment extraction | ✅ | ❌ | ❌ | ❌ | ❌ |
| Import/dep analysis | ✅ | ✅ | ✅ | ❌ | ❌ |
| SWUM expansion | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stereotype detection | ✅ | ⚠️ | ⚠️ | ⚠️ | ❌ |

**Legend:** ✅ Full | ⚠️ Partial/Generic | ❌ Missing

---

## Feature Definitions

### 1. File Discovery
Recognize source files by extension.

**Files to modify:** `src/validator.rs`, `src/generator.rs`

| Language | Extensions |
|----------|------------|
| Rust | `.rs` |
| TypeScript | `.ts`, `.tsx` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` |
| Python | `.py` |
| Go | `.go` |
| Swift | `.swift` |

### 2. Layer Inference
Classify files into architectural layers based on path/name patterns.

**File to modify:** `src/inference.rs`

| Layer | Rust | TS/JS | Python | Go | Swift |
|-------|------|-------|--------|-----|-------|
| 0 - Config | `Cargo.toml` | `package.json`, `tsconfig.json` | `pyproject.toml`, `setup.py` | `go.mod` | `Package.swift` |
| 1 - Entry | `main.rs`, `lib.rs` | `index.ts`, `main.ts`, `app.ts` | `__main__.py`, `app.py` | `main.go` | `main.swift`, `App.swift` |
| 2 - Domain | `src/*.rs` | `src/**/*.ts` | `src/**/*.py` | `internal/`, `pkg/` | `Sources/**/*.swift` |
| 3 - Utils | `**/utils.rs`, `**/helpers.rs` | `**/utils.ts`, `**/helpers.ts` | `**/utils.py`, `**/helpers.py` | `**/util.go` | `**/Utils.swift` |
| 4 - Tests | `tests/`, `*_test.rs` | `*.test.ts`, `*.spec.ts`, `__tests__/` | `test_*.py`, `*_test.py`, `tests/` | `*_test.go` | `*Tests.swift`, `Tests/` |

### 3. Export Extraction
Identify public API surface.

**File to modify:** `src/exports.rs`

| Language | Public Exports Pattern |
|----------|----------------------|
| Rust | `pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub const`, `pub type` |
| TypeScript | `export function`, `export class`, `export interface`, `export const`, `export type`, `export default` |
| JavaScript | `export function`, `export class`, `export const`, `module.exports`, `exports.` |
| Python | Functions/classes without `_` prefix, `__all__` list |
| Go | Capitalized identifiers (`func Foo`, `type Bar`, `const Baz`) |
| Swift | `public func`, `public class`, `public struct`, `public enum`, `open class` |

### 4. Doc Comment Extraction
Extract documentation for WHAT descriptions.

**File to modify:** `src/doc_extractor.rs`

| Language | Module Doc | Item Doc |
|----------|-----------|----------|
| Rust | `//!` at file top | `///` above item |
| TypeScript | `/** @module */` or first `/** */` | `/** */` above item (JSDoc) |
| JavaScript | `/** @module */` or first `/** */` | `/** */` above item (JSDoc) |
| Python | `"""docstring"""` at module top | `"""docstring"""` after `def`/`class` |
| Go | `// Package foo ...` at top | `// FuncName ...` above func |
| Swift | `///` or `/** */` at file top | `///` or `/** */` above item |

### 5. Import/Dependency Analysis
Parse import statements for dependency graph.

**File to modify:** `src/deps.rs`

| Language | Import Patterns |
|----------|----------------|
| Rust | `use crate::`, `use super::`, `mod foo;` |
| TypeScript | `import { } from './path'`, `import x from './path'` |
| JavaScript | `import`, `require('./path')` |
| Python | `from . import`, `from .module import`, `import .module` |
| Go | `import "package/path"`, `import "./relative"` |
| Swift | `import Module` (framework), internal refs via type usage |

### 6. Stereotype Detection
Classify files by architectural role based on imports/patterns.

**File to modify:** `src/stereotype.rs`

| Stereotype | Rust | TS/JS | Python | Go | Swift |
|------------|------|-------|--------|-----|-------|
| CLI | `use clap` | `commander`, `yargs` | `argparse`, `click`, `typer` | `flag`, `cobra` | `ArgumentParser` |
| HTTP Handler | `use axum/actix` | `express`, `fastify`, `koa` | `flask`, `fastapi`, `django` | `net/http`, `gin`, `echo` | `Vapor`, `URLSession` |
| Database | `use diesel/sqlx` | `prisma`, `typeorm`, `pg` | `sqlalchemy`, `psycopg` | `database/sql`, `gorm` | `CoreData`, `GRDB` |
| Test | `#[test]` | `describe`, `it`, `test(` | `pytest`, `unittest` | `func Test` | `XCTest` |

---

## Implementation Order

### Phase 1: Foundation (Low Effort, High Value)
1. **File discovery** - Add extensions to `validator.rs` source file list
2. **Layer inference** - Add config/entry/test patterns per language

### Phase 2: Core Features
3. **Export extraction** - Regex patterns per language
4. **Doc comment extraction** - Regex patterns per language

### Phase 3: Intelligence
5. **Import analysis** - Requires understanding module resolution
6. **Stereotype detection** - Framework-specific import patterns

---

## Implementation Details

### TypeScript/JavaScript

```rust
// exports.rs - TS/JS exports
fn extract_ts_exports(content: &str) -> BTreeSet<String> {
    let mut exports = BTreeSet::new();
    
    // export function foo() | export const foo | export class Foo
    let export_re = Regex::new(r"export\s+(?:async\s+)?(?:function|const|let|class|interface|type|enum)\s+(\w+)").unwrap();
    
    // export default
    let default_re = Regex::new(r"export\s+default\s+(?:function|class)?\s*(\w+)?").unwrap();
    
    // export { foo, bar }
    let named_re = Regex::new(r"export\s*\{([^}]+)\}").unwrap();
    
    for cap in export_re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            exports.insert(m.as_str().to_string());
        }
    }
    
    // ... similar for other patterns
    exports
}

// doc_extractor.rs - JSDoc
fn extract_jsdoc(content: &str) -> Option<String> {
    // Module-level: /** @module */ or first /** */
    let module_re = Regex::new(r"(?s)^[\s]*(/\*\*.*?\*/)").unwrap();
    
    if let Some(cap) = module_re.captures(content) {
        let doc = cap.get(1)?.as_str();
        return Some(clean_jsdoc(doc));
    }
    None
}

fn clean_jsdoc(doc: &str) -> String {
    doc.trim_start_matches("/**")
       .trim_end_matches("*/")
       .lines()
       .map(|l| l.trim().trim_start_matches('*').trim())
       .filter(|l| !l.starts_with('@'))  // Skip @param, @returns, etc.
       .collect::<Vec<_>>()
       .join(" ")
       .trim()
       .to_string()
}
```

### Python

```rust
// exports.rs - Python exports
fn extract_python_exports(content: &str) -> BTreeSet<String> {
    let mut exports = BTreeSet::new();
    
    // Check __all__ first (authoritative)
    let all_re = Regex::new(r#"__all__\s*=\s*\[([^\]]+)\]"#).unwrap();
    if let Some(cap) = all_re.captures(content) {
        let items = cap.get(1).unwrap().as_str();
        for item in items.split(',') {
            let name = item.trim().trim_matches(|c| c == '"' || c == '\'');
            if !name.is_empty() {
                exports.insert(name.to_string());
            }
        }
        return exports;
    }
    
    // Otherwise: public functions/classes (no _ prefix)
    let def_re = Regex::new(r"(?m)^(?:async\s+)?def\s+([a-zA-Z][a-zA-Z0-9_]*)").unwrap();
    let class_re = Regex::new(r"(?m)^class\s+([a-zA-Z][a-zA-Z0-9_]*)").unwrap();
    
    for cap in def_re.captures_iter(content) {
        let name = cap.get(1).unwrap().as_str();
        if !name.starts_with('_') {
            exports.insert(name.to_string());
        }
    }
    
    for cap in class_re.captures_iter(content) {
        let name = cap.get(1).unwrap().as_str();
        if !name.starts_with('_') {
            exports.insert(name.to_string());
        }
    }
    
    exports
}

// doc_extractor.rs - Python docstrings
fn extract_python_doc(content: &str) -> Option<String> {
    // Module docstring: first string literal at module level
    let doc_re = Regex::new(r#"^[\s]*(?:"""([\s\S]*?)"""|'''([\s\S]*?)''')"#).unwrap();
    
    if let Some(cap) = doc_re.captures(content) {
        let doc = cap.get(1).or_else(|| cap.get(2))?.as_str();
        return Some(doc.lines().next().unwrap_or("").trim().to_string());
    }
    None
}
```

### Go

```rust
// exports.rs - Go exports (capitalized = public)
fn extract_go_exports(content: &str) -> BTreeSet<String> {
    let mut exports = BTreeSet::new();
    
    // func FooBar() | func (r *Receiver) FooBar()
    let func_re = Regex::new(r"func\s+(?:\([^)]+\)\s+)?([A-Z][a-zA-Z0-9_]*)").unwrap();
    
    // type FooBar struct/interface
    let type_re = Regex::new(r"type\s+([A-Z][a-zA-Z0-9_]*)").unwrap();
    
    // const/var FooBar
    let const_re = Regex::new(r"(?:const|var)\s+([A-Z][a-zA-Z0-9_]*)").unwrap();
    
    for cap in func_re.captures_iter(content) {
        exports.insert(cap.get(1).unwrap().as_str().to_string());
    }
    for cap in type_re.captures_iter(content) {
        exports.insert(cap.get(1).unwrap().as_str().to_string());
    }
    for cap in const_re.captures_iter(content) {
        exports.insert(cap.get(1).unwrap().as_str().to_string());
    }
    
    exports
}

// doc_extractor.rs - Go doc comments
fn extract_go_doc(content: &str) -> Option<String> {
    // Package comment: // Package foo provides...
    let pkg_re = Regex::new(r"(?m)^// Package \w+ (.+)$").unwrap();
    
    if let Some(cap) = pkg_re.captures(content) {
        return Some(cap.get(1)?.as_str().to_string());
    }
    
    // Or block comment: /* Package foo ... */
    let block_re = Regex::new(r"(?s)/\*\s*Package \w+\s+(.+?)\*/").unwrap();
    if let Some(cap) = block_re.captures(content) {
        let doc = cap.get(1)?.as_str();
        return Some(doc.lines().next().unwrap_or("").trim().to_string());
    }
    
    None
}

// deps.rs - Go imports
fn extract_go_imports(content: &str, source_path: &str) -> Vec<(String, DepKind)> {
    let mut deps = Vec::new();
    
    // import "path" or import ( "path1" \n "path2" )
    let single_re = Regex::new(r#"import\s+"([^"]+)""#).unwrap();
    let block_re = Regex::new(r#"(?s)import\s*\(([^)]+)\)"#).unwrap();
    
    for cap in single_re.captures_iter(content) {
        let path = cap.get(1).unwrap().as_str();
        if let Some(resolved) = resolve_go_import(path, source_path) {
            deps.push((resolved, DepKind::Import));
        }
    }
    
    if let Some(cap) = block_re.captures(content) {
        let block = cap.get(1).unwrap().as_str();
        for line in block.lines() {
            let path = line.trim().trim_matches('"');
            if !path.is_empty() && !path.starts_with("//") {
                if let Some(resolved) = resolve_go_import(path, source_path) {
                    deps.push((resolved, DepKind::Import));
                }
            }
        }
    }
    
    deps
}
```

### Swift

```rust
// exports.rs - Swift exports
fn extract_swift_exports(content: &str) -> BTreeSet<String> {
    let mut exports = BTreeSet::new();
    
    // public/open func/class/struct/enum/protocol
    let pub_re = Regex::new(
        r"(?:public|open)\s+(?:final\s+)?(?:func|class|struct|enum|protocol|var|let)\s+(\w+)"
    ).unwrap();
    
    for cap in pub_re.captures_iter(content) {
        exports.insert(cap.get(1).unwrap().as_str().to_string());
    }
    
    exports
}

// doc_extractor.rs - Swift doc comments
fn extract_swift_doc(content: &str) -> Option<String> {
    // /// line comments or /** block */
    let line_re = Regex::new(r"(?m)^///\s*(.+)$").unwrap();
    let block_re = Regex::new(r"(?s)/\*\*(.+?)\*/").unwrap();
    
    // First doc comment in file
    if let Some(cap) = line_re.captures(content) {
        return Some(cap.get(1)?.as_str().trim().to_string());
    }
    if let Some(cap) = block_re.captures(content) {
        let doc = cap.get(1)?.as_str();
        return Some(doc.lines()
            .map(|l| l.trim().trim_start_matches('*').trim())
            .next()
            .unwrap_or("")
            .to_string());
    }
    
    None
}

// deps.rs - Swift imports (framework only, internal deps need type analysis)
fn extract_swift_imports(content: &str) -> Vec<(String, DepKind)> {
    let mut deps = Vec::new();
    
    let import_re = Regex::new(r"import\s+(\w+)").unwrap();
    
    for cap in import_re.captures_iter(content) {
        let module = cap.get(1).unwrap().as_str();
        // Skip system frameworks for internal dep graph
        if !is_system_framework(module) {
            deps.push((module.to_string(), DepKind::Import));
        }
    }
    
    deps
}

fn is_system_framework(name: &str) -> bool {
    matches!(name, 
        "Foundation" | "UIKit" | "SwiftUI" | "Combine" | "CoreData" |
        "XCTest" | "AppKit" | "CoreGraphics" | "Darwin" | "Dispatch"
    )
}
```

---

## Testing Strategy

Since you don't code in these languages, here's how to validate:

### 1. Golden File Tests
Create minimal example files for each language:

```
tests/fixtures/
  typescript/
    simple.ts        # export function foo() { }
    jsdoc.ts         # /** Module doc */ export class Bar { }
  python/
    simple.py        # def foo(): pass
    docstring.py     # """Module doc""" \n class Bar: pass
  go/
    simple.go        # func Foo() { }
    doc.go           # // Package foo provides... 
  swift/
    simple.swift     # public func foo() { }
    doc.swift        # /// Module documentation
```

### 2. Snapshot Tests
Run `semmap generate` on real open-source projects:

| Language | Test Repo |
|----------|-----------|
| TypeScript | `clone https://github.com/microsoft/TypeScript --depth 1` |
| Python | `clone https://github.com/pallets/flask --depth 1` |
| Go | `clone https://github.com/gin-gonic/gin --depth 1` |
| Swift | `clone https://github.com/vapor/vapor --depth 1` |

Snapshot the output, review manually once, then regression test.

### 3. Property Tests
Invariants that should hold for any language:
- Every discovered file appears in exactly one layer
- No duplicate paths
- Exports are alphabetically sorted
- Doc comments don't contain raw comment markers (`///`, `"""`, etc.)

### 4. Community Validation
Open GitHub issues asking users of each language to try `semmap generate` on their projects and report issues.

---

## File Modification Summary

| File | Changes |
|------|---------|
| `src/validator.rs` | Add `.swift` to `is_source_file` |
| `src/inference.rs` | Add layer patterns for all languages |
| `src/exports.rs` | Add `extract_X_exports()` for each language |
| `src/doc_extractor.rs` | Add `extract_X_doc()` for each language |
| `src/deps.rs` | Add `extract_X_imports()` for Go, Swift |
| `src/stereotype.rs` | Add framework detection patterns |
| `tests/` | Add fixture files and golden tests |

---

## Estimated Effort

| Phase | Effort | Deliverable |
|-------|--------|-------------|
| Phase 1 | 1 hour | File discovery + basic layer inference |
| Phase 2 | 4 hours | Export + doc extraction for all langs |
| Phase 3 | 4 hours | Import analysis + stereotype detection |
| Testing | 2 hours | Fixtures + snapshot tests |
| **Total** | **~11 hours** | Full parity |

---

## Open Questions

1. **Swift internal dependencies**: Swift uses type references, not explicit imports for internal modules. May need tree-sitter for accurate dep graph.

2. **Python relative imports**: `from ..foo import bar` resolution is complex. May need to track `__init__.py` hierarchy.

3. **Go module paths**: `import "github.com/user/repo/pkg"` - should we resolve to local vendor or just track external deps?

4. **Monorepo support**: TypeScript/JS projects often have `packages/` with multiple `package.json`. Layer inference needs workspace awareness.

---

## Success Criteria

Running `semmap generate` on a project in any supported language produces:
- ✅ Correct layer assignments
- ✅ Meaningful WHAT descriptions (from doc comments or SWUM)
- ✅ Accurate export lists
- ✅ Valid dependency graph (no false positives/negatives)
- ✅ No crashes or panics
