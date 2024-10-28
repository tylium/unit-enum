# UnitEnum

A procedural macro for enhancing unit-only enums with useful methods.

This crate provides the `UnitEnum` derive macro which automatically implements
various utility methods for working with unit variant enums. It respects the
enum's `#[repr]` attribute for discriminant types and provides methods for
accessing variant names, ordinals, and discriminants.

## Usage

```rust
use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
enum Status {
    Active = 1,
    Pending,    // 2
    Inactive = 5
}

// Access variant name
assert_eq!(Status::Active.name(), "Active");

// Get zero-based ordinal
assert_eq!(Status::Pending.ordinal(), 1);

// Convert from ordinal
assert_eq!(Status::from_ordinal(2), Some(Status::Inactive));

// Access discriminant value
assert_eq!(Status::Inactive.discriminant(), 5);

// Convert from discriminant
assert_eq!(Status::from_discriminant(1), Some(Status::Active));

// Get number of variants
assert_eq!(Status::len(), 3);

// Iterate over all variants
assert_eq!(
    Status::values().collect::<Vec<_>>(),
    vec![Status::Active, Status::Pending, Status::Inactive]
);
```

## Features

The `UnitEnum` derive macro provides the following methods:

- [`name()`](#method.name): Get the string name of a variant
- [`ordinal()`](#method.ordinal): Get the zero-based position of a variant
- [`from_ordinal()`](#method.from_ordinal): Convert an ordinal to a variant
- [`discriminant()`](#method.discriminant): Get the variant's discriminant value
- [`from_discriminant()`](#method.from_discriminant): Convert a discriminant to a variant
- [`len()`](#method.len): Get the total number of variants
- [`values()`](#method.values): Get an iterator over all variants

## Discriminant Types

The macro respects the enum's `#[repr]` attribute to determine discriminant types:

```rust
# use unit_enum::UnitEnum;
#[derive(UnitEnum)]
#[repr(u8)]
enum Small {
    A,  // 0u8
    B,  // 1u8
}

#[derive(UnitEnum)]
#[repr(i64)]
enum Large {
    X = -1_000_000,
    Y = 1_000_000,
}
```

Supported types include:
- `i8`, `i16`, `i32`, `i64`, `i128`
- `u8`, `u16`, `u32`, `u64`, `u128`

If no `#[repr]` is specified, `i32` is used by default.

## Limitations

- Only supports enums with unit variants
- Data-carrying variants or tuple variants will cause compile errors

## Generated Methods

The following methods are generated for any enum that derives `UnitEnum`:

```rust,ignore
impl EnumName {
    /// Returns the string name of the variant.
    pub fn name(&self) -> &str { ... }

    /// Returns the zero-based ordinal (position) of the variant.
    pub fn ordinal(&self) -> usize { ... }

    /// Converts an ordinal to its corresponding variant, if valid.
    pub fn from_ordinal(ord: usize) -> Option<Self> { ... }

    /// Returns the discriminant value of the variant.
    pub fn discriminant(&self) -> ReprType { ... }

    /// Converts a discriminant value to its corresponding variant, if valid.
    pub fn from_discriminant(discr: ReprType) -> Option<Self> { ... }

    /// Returns the total number of variants in the enum.
    pub fn len() -> usize { ... }

    /// Returns an iterator over all variants of the enum.
    pub fn values() -> impl Iterator<Item = Self> { ... }
}
```
