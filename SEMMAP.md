# project -- Semantic Map

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

## Layer 0 -- Config

`Cargo.toml`
Rust package manifest and dependencies. Centralizes project configuration.

`slopchop.toml`
Configuration for slopchop. Centralizes project configuration.

## Layer 1 -- Core

`src/lib.rs`
Library root and public exports. Provides application entry point.

`src/main.rs`
Validate a SEMMAP file. Provides application entry point.

## Layer 2 -- Domain

`src/commands.rs`
Implements commands functionality. Orchestrates business logic.
→ Exports: deps, generate, update, validate

`src/deps.rs`
Dependency analysis and layer violation detection. Parses input into structured data.
→ Exports: analyze, check_layer_violations, render_mermaid

`src/doc_extractor.rs`
Extracts documentation comments from source files. Supports application functionality.
→ Exports: extract_doc_comment

`src/error.rs`
Implements error functionality. Defines error types and handling.
→ Exports: ParseError, SemmapError, Severity, ValidationIssue, at_line, error, for_path, warning

`src/exports.rs`
Implements exports functionality. Supports application functionality.
→ Exports: extract_exports

`src/formatter.rs`
Implements formatter functionality. Formats data for output.
→ Exports: to_json, to_markdown, to_toml

`src/generator.rs`
Implements generator functionality. Supports application functionality.
→ Exports: GeneratorConfig, generate

`src/inference.rs`
Layer and description inference for SEMMAP generation. Supports application functionality.
→ Exports: infer_layer, infer_what, infer_why

`src/lang_python.rs`
Implements lang python. Parses input into structured data.
→ Exports: extract_imports

`src/parse_entries.rs`
Parses entries. Parses input into structured data.
→ Exports: parse_layer_entries

`src/parser.rs`
Implements parser functionality. Parses input into structured data.
→ Exports: parse

`src/stereotype.rs`
Stereotype classification for architectural role detection. Supports application functionality.
→ Exports: Stereotype, classify, stereotype_to_why

`src/swum.rs`
SWUM (Software Word Usage Model) for identifier expansion. Supports application functionality.
→ Exports: expand_identifier

`src/types.rs`
Implements types functionality. Defines domain data structures.
→ Exports: DepEdge, DepKind, DepNode, DependencyMap, Description, FileEntry, Layer, LegendEntry, SemmapFile, all_paths, find_entry, new, path_to_layer

`src/validator.rs`
Validates SEMMAP files for correctness and completeness. Supports application functionality.
→ Exports: ValidationResult, error_count, is_valid, validate, validate_against_codebase, warning_count

## Layer 3 -- Utilities

`src/commands/update_helpers.rs`
Updates helpers. Provides reusable helper functions.
→ Exports: add_new_entries, remove_deleted_entries

`src/path_utils.rs`
Computes the path prefix for entries based on root's position relative to `semmap_dir`. Provides reusable helper functions.
→ Exports: build_root_prefix, build_root_prefix_relative, prefix_path, strip_prefix_for_lookup

## Layer 4 -- Tests

`tests/deps_tests.rs`
Lower layer depending on higher layer should be flagged as violation. Verifies correctness.

`tests/generator_more_tests.rs`
Verify that public items (structs, functions, enums, traits) are correctly extracted into the exports field. Verifies correctness.
→ Exports: Auth, Role, User, login

`tests/generator_tests.rs`
Verify that the generator detects a Cargo.toml file and correctly assigns it to Layer 0 (Config). Verifies correctness.

`tests/parser_tests.rs`
Implements parser tests. Verifies correctness.

`tests/path_utils.rs`
Current directory should produce empty prefix. Verifies correctness.

`tests/update_more_tests.rs`
Updates more tests. Verifies correctness.

`tests/update_tests.rs`
Updates tests. Verifies correctness.

`tests/validator_tests.rs`
A valid SEMMAP with proper structure should pass validation. Verifies correctness.

