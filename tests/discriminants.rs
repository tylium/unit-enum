use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(u8)]
enum Flags {
    A,
    B,
    C = 1 << 3,
    D,
    E = 0x21 | 0x01,
    F,
    G = 0x41 ^ 0x01,
    H,
    I = 0xF0 & 0x30,
    J,
    K = 2 * 3,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(i16)]
enum Signed {
    A = -2,
    B,
    C,
    D = -(1 << 4),
    E,
}

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(u16)]
enum WithOther {
    A = 1 << 3,
    B,
    C,
    #[unit_enum(other)]
    Other(u16),
}

#[test]
fn discriminants_match_compiler() {
    for value in Flags::values() {
        assert_eq!(value.discriminant(), value as u8, "{}", value.name());
        assert_eq!(Flags::from_discriminant(value as u8), Some(value), "{}", value.name());
    }

    for value in Signed::values() {
        assert_eq!(value.discriminant(), value as i16, "{}", value.name());
        assert_eq!(Signed::from_discriminant(value as i16), Some(value), "{}", value.name());
    }
}

#[test]
fn implicit_discriminants_after_operator_expressions() {
    assert_eq!(Flags::B.discriminant(), 1);
    assert_eq!(Flags::D.discriminant(), 9);
    assert_eq!(Flags::F.discriminant(), 0x22);
    assert_eq!(Flags::H.discriminant(), 0x41);
    assert_eq!(Flags::J.discriminant(), 0x31);
    assert_eq!(Flags::L.discriminant(), 7);
    assert_eq!(Signed::C.discriminant(), 0);
    assert_eq!(Signed::E.discriminant(), -15);
}

#[test]
fn implicit_discriminants_with_other_variant() {
    assert_eq!(WithOther::B.discriminant(), 9);
    assert_eq!(WithOther::C.discriminant(), 10);
    assert_eq!(WithOther::from_discriminant(9), WithOther::B);
    assert_eq!(WithOther::from_discriminant(10), WithOther::C);
    assert_eq!(WithOther::from_discriminant(16), WithOther::Other(16));
}

#[test]
fn implicit_discriminants_in_const_context() {
    const D: u8 = Flags::D.discriminant();
    const FROM_D: Option<Flags> = Flags::from_discriminant(9);

    assert_eq!(D, 9);
    assert_eq!(FROM_D, Some(Flags::D));
}

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(C)]
enum ReprC {
    A,
    B = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(C, u8)]
enum ReprCWithInteger {
    A,
    #[unit_enum(other)]
    Other(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
#[repr(C)]
#[repr(u16)]
enum SplitRepr {
    A,
    #[unit_enum(other)]
    Other(u16),
}

#[test]
fn repr_hints_without_integer_type_default_to_i32() {
    let discriminant: i32 = ReprC::B.discriminant();

    assert_eq!(discriminant, ReprC::B as i32);
    assert_eq!(ReprC::from_discriminant(5), Some(ReprC::B));
}

#[test]
fn integer_type_is_found_among_repr_hints() {
    let combined: u8 = ReprCWithInteger::Other(7).discriminant();
    let split: u16 = SplitRepr::Other(7).discriminant();

    assert_eq!(combined, 7);
    assert_eq!(split, 7);
    assert_eq!(ReprCWithInteger::from_discriminant(0), ReprCWithInteger::A);
    assert_eq!(SplitRepr::from_discriminant(0), SplitRepr::A);
}
