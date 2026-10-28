# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Next

### Added

- Add a `VALUES` associated constant holding all unit variants in declaration order, usable in const contexts (e.g. `const ALL: [Status; Status::len()] = Status::VALUES;`)
- Support `#![no_std]` crates: generated code only depends on `core`

### Changed

- `name()` now returns `&'static str` instead of a `&str` borrowed from `self`
- `values()` iterates over `VALUES` instead of allocating a `Vec`

### Fixed

- Ignore `repr` hints that are not integer types, such as `C`: `#[repr(C)]` now uses `i32` and `#[repr(C, u8)]` (or separate `#[repr(C)]` and `#[repr(u8)]` attributes) uses `u8`, instead of failing to compile
- Use fully qualified paths in generated code so it compiles when names such as `Option`, `Some`, `None`, `Iterator` or `stringify!` are shadowed

## Version 1.5.1 (2026-10-28)

### Fixed

- Fix implicit discriminants that follow an explicit discriminant using an operator such as `<<` or `|` (e.g. with `A = 1 << 3, B`, `B.discriminant()` now returns `9` instead of `16`, and `from_discriminant(9)` returns `B`)
- Fix README examples that did not compile; they are now run as doctests

## Version 1.5.0 (2026-03-04)

### Added

- Make `name`, `ordinal`, `from_ordinal`, `discriminant`, `from_discriminant`, and `len` const-friendly by generating them as `const fn`
- Document the minimum supported Rust version (MSRV) as Rust 1.71 and add a section describing const usage in the documentation

## Version 1.4.3 (2025-05-20)

### Fixed

- Removed unnecessary type casts that were causing clippy warnings

## Version 1.4.2 (2025-05-17)

### Added

- Add documentation and funding links
- Setup GitHub actions

## Version 1.4.1 (2024-11-18)

### Fixed

- Update documentation examples to ignore code blocks so client tests are not impacted

## Version 1.4.0 (2024-10-28)

### Added

- Support for #[repr(type)]
- Support for Other variant

## Version 1.3.0 (2024-07-15)

### Added

- Add `name()` method

## Version 1.2.0 (2024-07-02)

### Added

- Add `discriminant()` and `from_discriminant(num)` methods

## Version 1.1.0 (2024-06-27)

### Added

- Add `values()` method and document all methods in UnitEnum macro

## Version 1.0.0 (2024-03-17)

### Added

- Initial Revision
