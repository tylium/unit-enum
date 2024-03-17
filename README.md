# UnitEnum

The `unit-enum` crate introduces a procedural macro `UnitEnum` for Rust developers working with enums that consist
entirely of unit variants. This macro enhances such enums by automatically implementing methods to manage enum variants
more effectively, promoting ease of use and reducing boilerplate code.

## Features

- **Ordinal Methods**: Easily obtain the ordinal of an enum variant or convert an ordinal value back to a corresponding
  enum variant.
- **Variant Count**: Retrieve the total number of variants in an enum.

## Installation

To use `unit-enum` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
unit-enum = "1.0.0"
```

## Quick Start

```rust
use unit_enum::UnitEnum;

#[derive(UnitEnum)]
enum Color {
    Red,
    Green,
    Blue,
}

let green = Color::Green;
assert_eq!(green.ordinal(), 1);
assert_eq!(Color::from_ordinal(1), Some(Color::Green));
assert_eq!(Color::len(), 3);
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on our [GitHub repository](#).

## License

This project is licensed under either of MIT or Apache-2.0, at your choice.