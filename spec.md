# SEMMAP Specification v0.2

## What It Is

A Markdown file that maps your codebase for LLMs. It prioritizes semantic intent over file structure using flat paths, rigorous descriptions, and architectural layering.

## Format Structure

```markdown
# project-name — Semantic Map

**Purpose:** One sentence describing the project goal.

## Legend

`[TAG]` Definition of the tag.

## Layer 0 — Config

`path/to/file.ext`
[What it does]. [Why it exists].
→ Exports: [Optional comma-separated list]
→ Touch: [Optional constraints, gotchas, or dependencies]

## Layer 1 — Core

...
```

## Description Rules

Every description **must** answer these questions in order, forming a single paragraph:

1.  **What** — What does this file do? (One sentence).
2.  **Why** — Why does it exist as a separate unit? What architectural role does it play?

Optional metadata lines follow the description:
*   `→ Exports:` Key structs, traits, or functions.
*   `→ Touch:` Critical context, side effects, or "read before modifying" warnings.

## Layer Schema

Organize files by abstraction level, not directory structure.

*   **Layer 0 — Config**: Build files, linters, CI, environment (`Cargo.toml`, `Dockerfile`).
*   **Layer 1 — Core**: Shared types, error definitions, utilities used everywhere.
*   **Layer 2 — Platform**: Hardware abstractions, OS interop, drivers, database connectors.
*   **Layer 3 — Domain**: Business logic, algorithms, rendering engines.
*   **Layer 4 — App**: Entry points, UI, event loops, wiring, integration tests.
*   **Layer 5 — Docs**: Readmes, specifications, roadmaps, architecture diagrams.

## Formatting Rules

1.  **Flat Paths**: Always use full paths from repo root (e.g., `crates/pal/src/lib.rs`).
2.  **No ASCII Trees**: Do not use `├─` characters.
3.  **Tags**: Use tags from the Legend in backticks on the path line (e.g., `[CRITICAL]`).
4.  **Formatting**: Markdown headers (`##`) for Layers.
5.  **Backticks**: Wrap code identifiers in descriptions (e.g., "Implements `MyTrait`").

## Example

```markdown
## Layer 2 — Platform

`crates/pal/src/vulkan/bridge.rs` `[CRITICAL]` `[UNSAFE]`
Wraps raw Vulkan handles into wgpu types via the HAL. Exists to enable the "Native Owns, WGPU Borrows" architecture where the app controls the device lifecycle.
→ Exports: `WgpuBridge`
→ Touch: Ensure `VulkanInstance` outlives the bridge to avoid use-after-free.
```

## CLI Commands

```bash
semmap init          # Generate skeleton from file tree
semmap validate      # Check paths exist and descriptions meet the "What/Why" rule
semmap drift         # List files in repo not covered by semmap
semmap patch <file>  # Apply a patch file
```

## Patch Format

```text
ADD src/foo.rs
layer: 2
tags: [UNSAFE]
desc: Implements the new buffer logic. Exists to optimize throughput.
exports: Buffer, Context
touch: Not thread-safe.

UPDATE src/existing.rs
desc: New description text here...

REMOVE src/old.rs
```
