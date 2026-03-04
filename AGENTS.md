# Agent Instructions

## Project Overview

This repository contains the `unit-enum` Rust crate, a procedural macro (`#[derive(UnitEnum)]`) for enums that are primarily composed of unit variants. It generates utility methods such as `name`, `ordinal`, `from_ordinal`, `discriminant`, `from_discriminant`, `len`, and `values`, while respecting the enum's `#[repr]` and optionally supporting a single `"other"` catch‑all variant.

## File Structure and Organization

- **AGENTS.md**: Root instruction file for AI agents (this file). Also used via symlinks by other tools.
- **README.md**: Human-facing crate documentation and quick-start examples.
- **Cargo.toml**: Crate metadata, dependencies, and configuration (`proc-macro = true`).
- **src/lib.rs**: Implementation of the `UnitEnum` derive macro and its validation / codegen logic.
- **src/lib.md**: Crate-level Rustdoc included into the library docs; keep examples and text in sync with `README.md`.
- **examples/**: Small runnable binaries (`example.rs`, `example_other.rs`) that demonstrate typical usage, including the optional `"other"` variant.
- **.github/workflows/rust.yml**: GitHub Actions workflow for CI (build/tests).
- **CHANGELOG.md**: Human-maintained change log; follow existing style when documenting externally visible changes.
- **LICENSE-MIT**, **LICENSE-APACHE**: Dual licensing information.
- **agents/**: Reserved for generic, reusable, and versioned instruction documents. Do not modify from within this project; treat contents (if any) as read-only.

## Symbolic Links

This AGENTS.md file is symlinked for compatibility with different AI agents:

- `CLAUDE.md` → `AGENTS.md`
- `.junie/guidelines.md` → `AGENTS.md`
- `GEMINI.md` → `AGENTS.md`
- `.cursorrules` → `AGENTS.md`

Any changes you make here automatically apply to those entrypoints; keep instructions tool-agnostic.

## Development Guidelines

### Code Organization

- Maintain a clear separation between agent instructions (`AGENTS.md`) and human documentation (`README.md`, `CHANGELOG.md`, crate docs in `src/lib.md`).
- Keep instructions dense, unambiguous, and optimized for AI processing (short sections, explicit bullets, minimal prose).
- When modifying macro behavior in `src/lib.rs`, also:
  - Update or add examples in `src/lib.md`, `README.md`, and `examples/` as appropriate.
  - Preserve the documented semantics of generated methods (`name`, `ordinal`, `discriminant`, etc.) unless the change is intentionally breaking and coordinated with versioning.
- Do not modify files under `agents/` from this repository; treat them as shared, versioned guidance.

### Rust & Testing

- Use `cargo test` (and, if relevant, `cargo test --examples`) to validate changes; this runs doctests in `src/lib.md` and `README.md` as well as any unit tests.
- Prefer adding or updating doctests and examples when fixing bugs or changing behavior in the derive macro.
- Avoid introducing public API changes without updating documentation and, when applicable, bumping versions and changelog entries in coordination with maintainers.

### File Management

- Only modify `AGENTS.md` directly; its symbolic links (`CLAUDE.md`, `.junie/guidelines.md`, `GEMINI.md`, `.cursorrules`) should not be edited independently.
- After making non-trivial changes to the crate or its structure, update this `AGENTS.md` to reflect the current project state so future agents have accurate context.
