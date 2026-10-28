//! Generated code must only rely on `core`, and not on names in scope where the enum is defined.
#![no_std]

extern crate std;

mod shadowed {
    #![allow(dead_code, unused_imports, unused_macros)]

    use unit_enum::UnitEnum;

    pub use self::Option::{None, Some};

    pub enum Option<T> {
        Some(T),
        None,
    }

    pub trait Iterator {}

    pub trait IntoIterator {}

    macro_rules! stringify {
        ($($tt:tt)*) => {
            "shadowed"
        };
    }

    #[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
    #[repr(u8)]
    pub enum Color {
        Red,
        Green,
        Blue = 10,
    }

    #[derive(Debug, Clone, Copy, PartialEq, UnitEnum)]
    #[repr(u8)]
    pub enum Status {
        Active = 1,
        #[unit_enum(other)]
        Unknown(u8),
    }
}

use shadowed::{Color, Status};

#[test]
fn unit_only_enum() {
    assert_eq!(Color::Blue.name(), "Blue");
    assert_eq!(Color::from_ordinal(1), Some(Color::Green));
    assert_eq!(Color::from_ordinal(3), None);
    assert_eq!(Color::from_discriminant(10), Some(Color::Blue));
    assert_eq!(Color::from_discriminant(2), None);
    assert_eq!(Color::VALUES, [Color::Red, Color::Green, Color::Blue]);
    assert!(Color::values().eq(Color::VALUES));
}

#[test]
fn enum_with_other_variant() {
    assert_eq!(Status::Unknown(7).name(), "Unknown");
    assert_eq!(Status::from_discriminant(1), Status::Active);
    assert_eq!(Status::from_discriminant(7), Status::Unknown(7));
    assert_eq!(Status::VALUES, [Status::Active]);
}
