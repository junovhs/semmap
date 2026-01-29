# issues-0001: Language Parity + Cleanup

---

## FORMAT (DO NOT MODIFY)

**Status values:** `OPEN`, `IN PROGRESS`, `DONE`, `DESCOPED`

**Issue format:**
```
## [N] Title
**Status:** OPEN
**Files:** list of files to modify

Description of the task.

**Resolution:** (fill when DONE) What was done, any notes.
```

**Instructions:**
- Work issues in order you feel is most important.
- Update status as you go
- Add **Resolution:** when completing
- Don't modify this FORMAT section
- Content below the line is the work. when done, archive in docs/archive and create next issues doc, either populated with issues are at least the template (issues-0002.md, issues-0003.md etc)

---

## [1] Remove `semmap update` command
...
**Status:** OPEN  
**Files:** `src/main.rs`, `src/commands.rs`, `src/commands/update_helpers.rs`, `tests/update_tests.rs`, `tests/update_more_tests.rs`

Delete the update command entirely. SEMMAP is a derived artifact now.

---

## [2] Add Swift/Go file discovery
**Status:** OPEN  
**Files:** `src/validator.rs`, `src/generator.rs`

Add `.swift`, `.go` to source file detection.

---

## [3] Layer inference for all languages
**Status:** OPEN  
**Files:** `src/inference.rs`

| Layer | TS/JS | Python | Go | Swift |
|-------|-------|--------|-----|-------|
| 0 | `package.json`, `tsconfig.json` | `pyproject.toml`, `setup.py` | `go.mod` | `Package.swift` |
| 1 | `index.ts`, `main.ts` | `__main__.py`, `app.py` | `main.go` | `main.swift` |
| 4 | `*.test.ts`, `*.spec.ts` | `test_*.py`, `*_test.py` | `*_test.go` | `*Tests.swift` |

---

## [4] Test fixtures for each language
**Status:** OPEN  
**Files:** `tests/fixtures/`

Create `go/simple.go`, `swift/simple.swift`, `python/simple.py`, `ts/simple.ts` with known exports/docs.

---

## [5] TypeScript export extraction
**Status:** OPEN  
**Files:** `src/exports.rs`

Match: `export function`, `export const`, `export class`, `export { }`, `export default`

---

## [6] Python export extraction
**Status:** OPEN  
**Files:** `src/exports.rs`

Check `__all__` first, else public functions/classes (no `_` prefix).

---

## [7] Go export extraction
**Status:** OPEN  
**Files:** `src/exports.rs`

Capitalized = public. Match `func Foo`, `type Bar`, `const Baz`.

---

## [8] Swift export extraction
**Status:** OPEN  
**Files:** `src/exports.rs`

Match `public`/`open` func/class/struct/enum.

---

## [9] TypeScript JSDoc extraction
**Status:** OPEN  
**Files:** `src/doc_extractor.rs`

First `/** */` block, strip `*` and `@tags`.

---

## [10] Python docstring extraction
**Status:** OPEN  
**Files:** `src/doc_extractor.rs`

Module-level `"""docstring"""`, return first line.

---

## [11] Go doc comment extraction
**Status:** OPEN  
**Files:** `src/doc_extractor.rs`

`// Package foo provides...` above package declaration.

---

## [12] Swift doc comment extraction
**Status:** OPEN  
**Files:** `src/doc_extractor.rs`

`///` or `/** */` style, same as Rust.

---

## [13] Go import analysis
**Status:** OPEN  
**Files:** `src/deps.rs`

Parse `import "path"` and `import ( )` blocks. Resolve against `go.mod`.

---

## [14] Swift import analysis
**Status:** OPEN | DESCOPED  
**Files:** `src/deps.rs`

Framework imports only (`import Foundation`). Internal deps need type resolution—out of scope.

---

## [15] Stereotype detection for all languages
**Status:** OPEN  
**Files:** `src/stereotype.rs`

| Stereotype | TS/JS | Python | Go | Swift |
|------------|-------|--------|-----|-------|
| CLI | `commander`, `yargs` | `click`, `typer` | `cobra` | `ArgumentParser` |
| HTTP | `express`, `fastify` | `flask`, `fastapi` | `gin` | `Vapor` |
| DB | `prisma`, `typeorm` | `sqlalchemy` | `gorm` | `CoreData` |

---

## Order
1. **[1]** first (cleanup)
2. **[4]** fixtures
3. **[2] + [3]** together (foundation)
4. **[5] + [9]** TS/JS
5. **[6] + [10]** Python
6. **[7] + [11] + [13]** Go
7. **[8] + [12] + [14]** Swift (14 descoped)
8. **[15]** stereotypes
