# semmap — Semantic Map

**Purpose:** Semantic Map generator and validator for codebases

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

## Layer 1 — Core

`lib.rs`
Library root and public exports. Separates concerns for maintainability.

`main.rs`
Application entry point. Separates concerns for maintainability.

## Layer 2 — Domain

`formatter.rs`
Implements formatter functionality. Separates concerns for maintainability.
→ Exports: to_markdown, to_json, to_toml

`deps.rs`
Implements deps functionality. Separates concerns for maintainability.
→ Exports: analyze, render_mermaid, check_layer_violations

`types.rs`
Implements types functionality. Isolates data structures for reuse across modules.
→ Exports: SemmapFile, LegendEntry, Layer, FileEntry, Description, DependencyMap, DepNode, DepEdge, DepKind, new, all_paths, find_entry, path_to_layer, new, new, new

`parser.rs`
Implements parser functionality. Separates concerns for maintainability.
→ Exports: parse

`error.rs`
Implements error functionality. Provides unified error handling.
→ Exports: SemmapError, ParseError, ValidationIssue, Severity, error, warning, at_line, for_path

`generator.rs`
Implements generator functionality. Separates concerns for maintainability.
→ Exports: GeneratorConfig, generate

`validator.rs`
Implements validator functionality. Separates concerns for maintainability.
→ Exports: ValidationResult, is_valid, error_count, warning_count, validate, validate_against_codebase

