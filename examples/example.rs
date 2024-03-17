use unit_enum::UnitEnum;

#[derive(Debug, Clone, Copy, UnitEnum)]
enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    println!("Ordinal of Green: {:?}", Color::Green.ordinal());    
    println!("Value of ordinal 2: {:?}", Color::from_ordinal(2));    
    println!("Number of Color variants: {:?}", Color::len());    
}