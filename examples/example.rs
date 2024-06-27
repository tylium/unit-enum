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