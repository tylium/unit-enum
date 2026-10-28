# Agent Instructions

## Project Overview

This repository contains the `unit-enum` Rust crate, a procedural macro (`#[derive(UnitEnum)]`) for enums that are primarily composed of unit variants. It generates utility methods such as `name`, `ordinal`, `from_ordinal`, `discriminant`, `from_discriminant`, `len`, and `values`, while respecting the enum's `#[repr]` and optionally supporting a single `"other"` catch‑all variant.

## File Structure and Organization

- **AGENTS.md**: Root instruction file for AI agents (this file). Read directly by all supported tools; there are no tool-specific copies or symlinks.
- **README.md**: Human-facing crate documentation and quick-start examples. Compiled as doctests via the `#[cfg(doctest)] ReadmeDoctests` item in `src/lib.rs`; every `rust` block must build.
- **Cargo.toml**: Crate metadata, dependencies, and configuration (`proc-macro = true`).
- **src/lib.rs**: Implementation of the `UnitEnum` derive macro and its validation / codegen logic.
- **src/lib.md**: Crate-level Rustdoc included into the library docs; keep examples and text in sync with `README.md`.
- **examples/**: Small runnable binaries (`example.rs`, `example_other.rs`) that demonstrate typical usage, including the optional `"other"` variant.
- **tests/**: Integration tests for generated code (`discriminants.rs`: discriminant computation, checked against the compiler's `as` casts).
- **.github/workflows/rust.yml**: GitHub Actions workflow for CI (build/tests).
- **CHANGELOG.md**: Human-maintained change log; follow existing style when documenting externally visible changes.
- **LICENSE-MIT**, **LICENSE-APACHE**: Dual licensing information.
- **agents/**: Reserved for generic, reusable, and versioned instruction documents. Do not modify from within this project; treat contents (if any) as read-only.

## Development Guidelines

### Code Organization

- Maintain a clear separation between agent instructions (`AGENTS.md`) and human documentation (`README.md`, `CHANGELOG.md`, crate docs in `src/lib.md`).
- Keep instructions dense, unambiguous, and optimized for AI processing (short sections, explicit bullets, minimal prose).
- When modifying macro behavior in `src/lib.rs`, also:
  - Update or add examples in `src/lib.md`, `README.md`, and `examples/` as appropriate.
  - Preserve the documented semantics of generated methods (`name`, `ordinal`, `discriminant`, etc.) unless the change is intentionally breaking and coordinated with versioning.
- Do not modify files under `agents/` from this repository; treat them as shared, versioned guidance.

### Rust & Testing

- Use `cargo test` (and, if relevant, `cargo test --examples`) to validate changes; this runs doctests in `src/lib.md` and `README.md` as well as the integration tests in `tests/`.
- Add regression tests for macro bugs to `tests/`; for unit-only enums, compare generated values with `Variant as Repr`.
- Prefer adding or updating doctests and examples when fixing bugs or changing behavior in the derive macro.
- Avoid introducing public API changes without updating documentation and, when applicable, bumping versions and changelog entries in coordination with maintainers.

### File Management

- Keep agent instructions in `AGENTS.md` only and tool-agnostic; do not add tool-specific files or symlinks (`CLAUDE.md`, `GEMINI.md`, `.cursorrules`, `.junie/guidelines.md`).
- After making non-trivial changes to the crate or its structure, update this `AGENTS.md` to reflect the current project state so future agents have accurate context.
