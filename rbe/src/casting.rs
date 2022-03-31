#[allow(overflowing_literals)]
fn main() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("128 as a i16 is {}", 128 as i16);
    println!("128 as a i8 is {}", 128 as i8);
}
