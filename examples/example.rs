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