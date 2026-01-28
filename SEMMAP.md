# project — Semantic Map

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

## Layer 0 — Config

`Cargo.toml`
Rust package manifest and dependencies. Centralizes build and runtime configuration.

`slopchop.toml`
Handles slopchop. Separates concerns for maintainability.

## Layer 1 — Core

`src\lib.rs`
Library root and public exports. Separates concerns for maintainability.

`src\main.rs`
Application entry point. Separates concerns for maintainability.

## Layer 2 — Domain

`src\commands.rs`
Implements commands functionality. Separates concerns for maintainability.
→ Exports: validate, generate, deps, update

`src\deps.rs`
Implements deps functionality. Separates concerns for maintainability.
→ Exports: analyze, render_mermaid, check_layer_violations

`src\error.rs`
Implements error functionality. Provides unified error handling.
→ Exports: SemmapError, ParseError, ValidationIssue, Severity, error, warning, at_line, for_path

`src\exports.rs`
Implements exports functionality. Separates concerns for maintainability.
→ Exports: extract_exports

`src\formatter.rs`
Implements formatter functionality. Separates concerns for maintainability.
→ Exports: to_markdown, to_json, to_toml

`src\generator.rs`
Implements generator functionality. Separates concerns for maintainability.
→ Exports: GeneratorConfig, generate

`src\inference.rs`
Implements inference functionality. Separates concerns for maintainability.
→ Exports: infer_layer, infer_what, infer_why

`src\lang_python.rs`
Implements lang_python functionality. Separates concerns for maintainability.
→ Exports: extract_imports

`src\parser.rs`
Implements parser functionality. Separates concerns for maintainability.
→ Exports: parse

`src\parse_entries.rs`
Implements parse_entries functionality. Separates concerns for maintainability.
→ Exports: parse_layer_entries

`src\types.rs`
Implements types functionality. Isolates data structures for reuse across modules.
→ Exports: SemmapFile, LegendEntry, Layer, FileEntry, Description, DependencyMap, DepNode, DepEdge, DepKind, new, all_paths, find_entry, path_to_layer, new, new, new

`src\validator.rs`
Implements validator functionality. Separates concerns for maintainability.
→ Exports: ValidationResult, is_valid, error_count, warning_count, validate, validate_against_codebase

## Layer 3 — Utilities

`src\path_utils.rs`
Implements path_utils functionality. Separates concerns for maintainability.
→ Exports: build_root_prefix, prefix_path, strip_prefix_for_lookup

`tests\path_utils.rs`
Implements path_utils functionality. Separates concerns for maintainability.

## Layer 4 — Tests

`tests\deps_tests.rs`
Implements deps_tests functionality. Separates concerns for maintainability.

`tests\generator_tests.rs`
Implements generator_tests functionality. Separates concerns for maintainability.
→ Exports: User, login, Role, Auth

`tests\parser_tests.rs`
Implements parser_tests functionality. Separates concerns for maintainability.

`tests\validator_tests.rs`
Implements validator_tests functionality. Separates concerns for maintainability.

