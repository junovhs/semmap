**66% kill rate.** That's your new headline. Here's a README that matches the engineering rigor you just proved:

```markdown
# semmap — Semantic Maps for AI-Native Codebases

[![Mutation Testing](https://img.shields.io/badge/mutation%20coverage-66%25-blue)](https://github.com/junovhs/semmap/actions)
[![CI](https://github.com/junovhs/semmap/workflows/Tests/badge.svg)](https://github.com/junovhs/semmap/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**AI generates code. Semmap verifies it doesn't lie about what it does.**

Semmap creates living documentation that maps your codebase by **semantic intent** (what files do and why they exist) rather than just file structure. It enforces architectural layering, detects dependency violations, and ensures your AI-generated code actually matches your mental model.

Built with [deterministic verification](https://github.com/junovhs/semmap/blob/main/docs/mutation-testing.md) — because "it works on my machine" isn't good enough when AI is writing the code.

## Why Semmap Exists

When AI writes code:
1. It generates plausible-looking functions that might be subtly wrong
2. It doesn't document architectural intent (layers, dependencies, contracts)
3. It drifts from the original design over time
4. You can't trust static analysis alone (it passes even when logic is inverted)

Semmap solves this by:
- **Generating** semantic documentation from code (`semmap generate`)
- **Validating** that code matches documentation (`semmap validate`)
- **Enforcing** architectural layers (Layer 0 Config → Layer 4 Tests)
- **Detecting** dependency violations (lower layers depending on higher layers)
- **Verifying** description quality with mutation testing (66% coverage)

## Installation

```bash
cargo install semmap
```

Or use the pre-built binaries from [releases](https://github.com/junovhs/semmap/releases).

## Quick Start

```bash
# Generate a semantic map from your codebase
semmap generate --output SEMMAP.md

# Validate that your code matches the map
semmap validate SEMMAP.md

# Check for architectural violations (lower layers depending on higher layers)
semmap deps SEMMAP.md --check

# Update the map when files change
semmap update SEMMAP.md
```

## The SEMMAP Format

A SEMMAP file organizes your codebase by **architectural layer**, not directory structure:

```markdown
# myproject — Semantic Map

**Purpose:** A semantic documentation generator for AI-coded projects.

## Legend

`[ENTRY]` Application entry point  
`[CORE]` Core business logic  
`[TYPE]` Data structures and types  
`[UTIL]` Utility functions  

## Layer 0 — Config

`Cargo.toml`  
Rust package manifest and dependencies. Centralizes project configuration.

## Layer 1 — Core

`src/lib.rs` `[ENTRY]`  
Library root and public exports. Provides application entry point.

## Layer 2 — Domain

`src/parser.rs` `[CORE]`  
Parses SEMMAP markdown into structured data. Parses input into structured data.  
→ Exports: parse

`src/validator.rs` `[CORE]`  
Validates SEMMAP files for correctness and completeness. Supports application functionality.  
→ Exports: validate, validate_against_codebase

## Layer 3 — Utilities

`src/path_utils.rs` `[UTIL]`  
Path prefix computation for entry mapping. Provides reusable helper functions.  
→ Exports: build_root_prefix, strip_prefix_for_lookup

## Layer 4 — Tests

`tests/parser_tests.rs`  
Implements parser tests. Verifies correctness.
```

## SWUM: Semantic Word Usage Model

Semmap uses SWUM (Software Word Usage Model) to expand cryptic identifiers into human-readable descriptions:

| Identifier | Expansion |
|------------|-----------|
| `get_user_profile` | "Gets the user profile." |
| `is_valid` | "Checks if valid." |
| `render_page` | "Formats page for output." |
| `update_cache` | "Updates cache." |

SWUM understands 40+ verb patterns and handles `snake_case`, `camelCase`, and `PascalCase` automatically.

## Architectural Enforcement

Semmap enforces a strict 5-layer architecture:

- **Layer 0** — Config: Build files, manifests, environment
- **Layer 1** — Core: Entry points, public APIs
- **Layer 2** — Domain: Business logic, parsers, validators
- **Layer 3** — Utilities: Helpers, utils, common code
- **Layer 4** — Tests: Test suites, fixtures

**The Rule:** Lower layers cannot depend on higher layers. Semmap detects violations like "Layer 2 (Domain) importing from Layer 3 (Utils)".

## Multi-Language Support

| Language | Discovery | Layer Inference | Export Extraction | Doc Extraction | Import Analysis |
|----------|:---------:|:---------------:|:-----------------:|:--------------:|:---------------:|
| **Rust** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **TypeScript/JavaScript** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Python** | ✅ | ✅ | ✅ | ⚠️ | ✅ |
| **Go** | ✅ | ✅ | ✅ | ⚠️ | ⚠️ |
| **Swift** | ✅ | ❌ | ❌ | ❌ | ❌ |

See [Language Parity Plan](docs/lang-parity-plan.md) for roadmap.

## Deterministic Verification

Semmap is tested with [cargo-mutants](https://github.com/sourcefrog/cargo-mutants) to ensure behavioral correctness:

- **66% total kill rate** (industry average: 30-50%)
- **308 mutants generated** (artificial bugs injected into code)
- **192 caught** by tests (62%)
- **11 timeouts** (infinite loops prevented, 4%)

This means when semmap says your code is valid, it's not just passing smoke tests—it's been verified against hundreds of failure scenarios.

## CI Integration

Add to your GitHub Actions:

```yaml
- name: Validate SEMMAP
  run: |
    cargo install semmap
    semmap validate SEMMAP.md --strict
    semmap deps SEMMAP.md --check
```

Or use the [semmap-action](https://github.com/marketplace/actions/semmap-validate) (coming soon).

## Philosophy

Semmap was built by a product designer who appreciates architectural rigor, not a dev. It exists because:

1. **AI coding is probabilistic** — Semmap makes it (more) deterministic
2. **Documentation drifts** — Semmap validates it stays true
3. **Architecture erodes** — Semmap enforces layer boundaries
4. **Trust is earned** — Semmap proves correctness with mutation testing

We are approaching a future where you don't need to be a systems programmer to build systems-level tooling. With the advent of AI coding, now more than ever you need to care about correctness more than cleverness.

## Contributing

We welcome contributions, especially for:
- Language support (Go, Swift, Python docstrings)
- Editor integrations (VS Code, Vim)
- CI/CD templates

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — See [LICENSE](LICENSE).

---
