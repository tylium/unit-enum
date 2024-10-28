use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
enum Color {
    Red = 10,
    Green,
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