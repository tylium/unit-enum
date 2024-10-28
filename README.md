# UnitEnum Crate Documentation

The `unit-enum` crate provides a procedural macro `UnitEnum` designed to enhance enums in Rust, particularly those
consisting solely of unit variants. This macro simplifies working with such enums by providing useful utility methods.

## Features

- `name`: Retrieve the name of an enum variant.
- `ordinal`: Retrieve the ordinal of an enum variant, starting from 0.
- `from_ordinal`: Convert an ordinal back to an enum variant, if possible.
- `discriminant`: Retrieve the discriminant of an enum variant.
- `from_discriminant`: Convert a discriminant back to an enum variant.
- `len`: Get the total number of variants in the enum.
- `values`: Returns an iterator over all variants of the enum, allowing for easy iteration and handling of each variant.

## Limitations

- Applicable only to enums with unit variants.
- Enums with data-carrying or tuple variants are not supported and will result in a compile-time error.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
unit-enum = "1.4.0"
```

## Quick Start

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

    // Get the total number of variants
    assert_eq!(Color::len(), 3);

    // Iterate over all variants
    assert_eq!(
        Color::values().collect::<Vec<_>>(),
        vec![Color::Red, Color::Green, Color::Blue]
    );
}
```

## Discriminant Types

The crate respects the enum's `#[repr]` attribute to determine the type of discriminant values. Supported types include:
- `#[repr(i8)]`, `#[repr(i16)]`, `#[repr(i32)]`, `#[repr(i64)]`, `#[repr(i128)]`
- `#[repr(u8)]`, `#[repr(u16)]`, `#[repr(u32)]`, `#[repr(u64)]`, `#[repr(u128)]`

If no `#[repr]` attribute is specified, the discriminant type defaults to `i32`.

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
    C = 1_000_000_000
}
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on our [GitHub repository](https://github.com/tylium/unit-enum).

## License

This project is licensed under either of MIT or Apache-2.0, at your choice.