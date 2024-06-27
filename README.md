# UnitEnum Crate Documentation

The `unit-enum` crate provides a procedural macro `UnitEnum` designed to enhance enums in Rust, particularly those
consisting solely of unit variants. This macro simplifies working with such enums by providing methods to convert
between enum variants and their ordinal positions, along with utility methods to count the number of variants and iterate over them.

## Features

- `ordinal`: Retrieve the ordinal of an enum variant, starting from 0.
- `from_ordinal`: Convert an ordinal back to an enum variant, if possible.
- `len`: Get the total number of variants in the enum.
- `values`: Returns an iterator over all variants of the enum, allowing for easy iteration and handling of each variant.

## Limitations

- Applicable only to enums with unit variants.
- Enums with data-carrying or tuple variants are not supported and will result in a compile-time error.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
unit-enum = "1.1.0"
```

## Quick Start

```rust
use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
enum Color {
  Red,
  Green,
  Blue
}

fn main() {
  println!("Ordinal of Green: {:?}", Color::Green.ordinal());
  // Ordinal of Green: 1

  println!("Value of ordinal 2: {:?}", Color::from_ordinal(2));
  // Value of ordinal 2: Some(Blue)

  println!("Number of Color variants: {:?}", Color::len());
  // Number of Color variants: 3

  println!("List of Color variants: {:?}", Color::values().collect::<Vec<_>>());
  // List of Color variants: [Red, Green, Blue]
}
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on our [GitHub repository](#).

## License

This project is licensed under either of MIT or Apache-2.0, at your choice.