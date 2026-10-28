use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
enum Color {
    Red = 10,
    Green,
    Blue = 45654,
}

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(u16)]
enum Status {
    Active = 1,
    Inactive = 2,
    #[unit_enum(other)]
    Unknown(u16),
}

fn static_name(color: Color) -> &'static str {
    color.name()
}

#[test]
fn name_outlives_value() {
    assert_eq!(static_name(Color::Green), "Green");

    const NAME: &str = Status::Unknown(3).name();
    assert_eq!(NAME, "Unknown");
}

#[test]
fn values_const_matches_iterator() {
    const ALL: [Color; Color::len()] = Color::VALUES;

    assert_eq!(ALL, [Color::Red, Color::Green, Color::Blue]);
    assert!(Color::values().eq(ALL));
    assert_eq!(Status::VALUES, [Status::Active, Status::Inactive]);
    assert!(Status::values().eq(Status::VALUES));
}

#[test]
fn values_index_by_ordinal() {
    for (ordinal, value) in Color::VALUES.into_iter().enumerate() {
        assert_eq!(value.ordinal(), ordinal);
        assert_eq!(Color::from_ordinal(ordinal), Some(value));
    }
}
