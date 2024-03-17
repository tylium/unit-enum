# UnitEnum Crate Documentation

The `unit-enum` crate provides a procedural macro `UnitEnum` designed to enhance enums in Rust, particularly those
consisting solely of unit variants. This macro simplifies working with such enums by providing methods to convert
between enum variants and their ordinal positions, along with a utility method to count the number of variants.

## Features

- `ordinal`: Retrieve the ordinal of an enum variant, starting from 0.
- `from_ordinal`: Convert an ordinal back to an enum variant, if possible.
- `len`: Get the total number of variants in the enum.

## Limitations

- Applicable only to enums with unit variants.
- Enums with data-carrying or tuple variants are not supported and will result in a compile-time error.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
unit-enum = "1.0.0"
```

## Usage

First, derive your enum with `UnitEnum`:

```rust
use unit_enum::UnitEnum;

#[derive(UnitEnum)]
enum MyEnum {
    VariantOne,
    VariantTwo,
}
```

Then, you can use the provided methods as follows:

```rust
let variant = MyEnum::VariantOne;
let ordinal = variant.ordinal();  // Gets the ordinal of the variant
let variant_from_ordinal = MyEnum::from_ordinal(ordinal).unwrap(); // Converts an ordinal back to an enum variant
assert_eq!(variant, variant_from_ordinal);
let total_variants = MyEnum::len();  // Gets the total number of variants
```