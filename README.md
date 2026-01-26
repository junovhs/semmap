# SEMMAP CLI

A Rust CLI tool for generating, validating, and managing Semantic Maps for codebases.

## What is SEMMAP?

SEMMAP (Semantic Map) is a Markdown documentation format that maps your codebase for LLMs. It prioritizes semantic intent over file structure using flat paths, rigorous descriptions, and architectural layering.

## Installation

```bash
cargo build --release
# Binary will be at ./target/release/semmap
```

## Usage

### Generate a SEMMAP from your codebase

```bash
semmap generate --root ./src --name "myproject" --purpose "A description of your project"
```

Options:
- `--root` / `-r`: Root directory to scan (default: `.`)
- `--output` / `-o`: Output file path (default: `SEMMAP.md`)
- `--name`: Project name (defaults to directory name)
- `--purpose`: Project purpose statement
- `--format`: Output format: `md`, `json`, `toml` (default: `md`)

### Validate a SEMMAP

```bash
semmap validate --file SEMMAP.md --root ./src
```

Options:
- `--file` / `-f`: Path to SEMMAP file (default: `SEMMAP.md`)
- `--root` / `-r`: Root directory to check file existence (default: `.`)
- `--strict`: Also check for undocumented files in the codebase

### Analyze Dependencies

```bash
semmap deps --file SEMMAP.md --root ./src
```

Options:
- `--file` / `-f`: Path to SEMMAP file (default: `SEMMAP.md`)
- `--root` / `-r`: Root directory of codebase (default: `.`)
- `--format`: Output format: `mermaid`, `json` (default: `mermaid`)
- `--check`: Check for layer violations (lower layers should not depend on higher layers)

### Update an Existing SEMMAP

```bash
semmap update --file SEMMAP.md --root ./src
```

This will:
- Add entries for new files
- Remove entries for deleted files
- Preserve your existing descriptions

## SEMMAP Format

```markdown
# project-name — Semantic Map

**Purpose:** One sentence describing the project goal.

## Legend

`[TAG]` Definition of the tag.

## Layer 0 — Config

`path/to/file.ext`
[What it does]. [Why it exists].
→ Exports: Optional comma-separated list
→ Touch: Optional constraints or gotchas

## Layer 1 — Core

...
```

### Layer Schema

| Layer | Name | Purpose |
|-------|------|---------|
| 0 | Config | Build, runtime configuration |
| 1 | Core | Entry points, library roots |
| 2 | Domain | Core business logic, types |
| 3 | Utilities | Helper functions, common code |
| 4 | Tests | Test files |

### Description Rules

Every description must answer:
1. **What** — What does this file do? (One sentence ending with period)
2. **Why** — Why does it exist as a separate unit?

## Integration with SlopChop

SEMMAP pairs well with [SlopChop](https://github.com/...) for enforcing code quality. Use SEMMAP to document architecture and SlopChop to enforce it.

## License

MIT
