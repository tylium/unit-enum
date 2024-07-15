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
unit-enum = "1.3.0"
```

## Quick Start

```rust
use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
enum Color {
    Red = 10,
    Green,
    Blue = 45654
}

fn main() {
    println!("Name of Blue: {:?}", Color::Blue.name());
    // Name of Blue: "Blue"

    println!("Ordinal of Green: {:?}", Color::Green.ordinal());
    // Ordinal of Green: 1

    println!("Value of ordinal 2: {:?}", Color::from_ordinal(2));
    // Value of ordinal 2: Some(Blue)

    println!("Value of ordinal 4: {:?}", Color::from_ordinal(4));
    // Value of ordinal 4: None

    println!("Discriminant of Blue: {:?}", Color::Blue.discriminant());
    // Discriminant of Blue: 45654

    println!("Discriminant of Green: {:?}", Color::Green.discriminant());
    // Discriminant of Green: 11

    println!("Value of discriminant 10: {:?}", Color::from_discriminant(10));
    // Value of discriminant 10: Some(Red)

    println!("Value of discriminant 0: {:?}", Color::from_discriminant(0));
    // Value of discriminant 0: None

    println!("Number of Color variants: {:?}", Color::len());
    // Number of Color variants: 3

    println!("List of Color variants: {:?}", Color::values().collect::<Vec<_>>());
    // List of Color variants: [Red, Green, Blue]
}
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on our [GitHub repository](https://github.com/tylium/unit-enum).

## License

This project is licensed under either of MIT or Apache-2.0, at your choice.