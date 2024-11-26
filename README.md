# unit-enum

[![Crates.io](https://img.shields.io/crates/v/unit-enum.svg)](https://crates.io/crates/unit-enum)
[![Documentation](https://docs.rs/unit-enum/badge.svg)](https://docs.rs/unit-enum)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md#license)

The `unit-enum` crate provides a procedural macro `UnitEnum` designed to enhance enums in Rust, particularly those
consisting of unit variants. This macro simplifies working with such enums by providing useful utility methods.

## Features

- `name`: Retrieve the name of an enum variant.
- `ordinal`: Retrieve the ordinal of an enum variant, starting from 0.
- `from_ordinal`: Convert an ordinal back to an enum variant, if possible.
- `discriminant`: Retrieve the discriminant of an enum variant.
- `from_discriminant`: Convert a discriminant back to an enum variant.
- `len`: Get the total number of unit variants in the enum (excluding the "other" variant if present).
- `values`: Returns an iterator over all unit variants of the enum.

## Supported Enum Types

The macro supports two types of enums:
1. Enums with only unit variants
2. Enums with unit variants plus one "other" variant for handling undefined discriminant values

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
unit-enum = "1.4.1"
```

## Quick Start

### Basic Usage (Unit Variants Only)

```rust
use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(i16)]  // Specify the discriminant type (optional, defaults to i32)
enum Color {
    Red = 10,
    Green,     // 11
    Blue = 45654
}

fn main() {
    // Get the name of a variant
    assert_eq!(Color::Blue.name(), "Blue");

    // Get the ordinal (position) of a variant
    assert_eq!(Color::Green.ordinal(), 1);

    // Convert from ordinal back to variant
    assert_eq!(Color::from_ordinal(2), Some(Color::Blue));
    assert_eq!(Color::from_ordinal(4), None);

    // Get the discriminant value (respects the repr type)
    assert_eq!(Color::Blue.discriminant(), 45654);
    assert_eq!(Color::Green.discriminant(), 11);

    // Convert from discriminant back to variant
    assert_eq!(Color::from_discriminant(10), Some(Color::Red));
    assert_eq!(Color::from_discriminant(0), None);

    // Get the total number of unit variants
    assert_eq!(Color::len(), 3);

    // Iterate over all variants
    assert_eq!(
        Color::values().collect::<Vec<_>>(),
        vec![Color::Red, Color::Green, Color::Blue]
    );
}
```

### Usage with "Other" Variant

```rust
use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(u16)]  // repr attribute is required when using an "other" variant
enum Status {
    Active = 1,
    Inactive = 2,
    #[unit_enum(other)]
    Unknown(u16),  // type must match repr
}

fn main() {
    // from_discriminant always returns a value when "other" variant is present
    assert_eq!(Status::from_discriminant(1), Status::Active);
    assert_eq!(Status::from_discriminant(42), Status::Unknown(42));

    // ordinal treats "other" as the last variant
    assert_eq!(Status::Active.ordinal(), 0);
    assert_eq!(Status::Unknown(42).ordinal(), 2);

    // len returns only the number of unit variants
    assert_eq!(Status::len(), 2);

    // values iterates only over unit variants
    assert_eq!(
        Status::values().collect::<Vec<_>>(),
        vec![Status::Active, Status::Inactive]
    );
}
```

## Discriminant Types

The crate respects the enum's `#[repr]` attribute to determine the type of discriminant values. Supported types include:
- `#[repr(i8)]`, `#[repr(i16)]`, `#[repr(i32)]`, `#[repr(i64)]`, `#[repr(i128)]`
- `#[repr(u8)]`, `#[repr(u16)]`, `#[repr(u32)]`, `#[repr(u64)]`, `#[repr(u128)]`

If no `#[repr]` attribute is specified, the discriminant type defaults to `i32`. Note that when using an "other" variant, the `#[repr]` attribute is required and must match the type of the "other" variant's field.

```rust
#[derive(UnitEnum)]
#[repr(u8)]  // Use u8 for discriminants
enum SmallEnum {
    A,  // 0u8
    B,  // 1u8
    C   // 2u8
}

#[derive(UnitEnum)]
#[repr(i64)]  // Use i64 for large discriminants
enum LargeEnum {
    A = -1_000_000,
    B = 5_000_000,
    #[unit_enum(other)]
    Other(i64)  // type matches repr
}
```

## Requirements for "Other" Variant

When using an "other" variant, the following requirements must be met:
- The enum must have a `#[repr(type)]` attribute
- Only one variant can be marked with `#[unit_enum(other)]`
- The "other" variant must have exactly one unnamed field matching the repr type
- All other variants must be unit variants

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on our [GitHub repository](https://github.com/tylium/unit-enum).

## License

This project is licensed under either of MIT or Apache-2.0, at your choice.
