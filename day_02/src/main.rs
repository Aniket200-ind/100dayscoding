fn main() {
    // ! Integer type
    // ? Bits contain two types : Signed(i) and Unsigned(u) containing 8, 16, 32, 64, 128, arch(bits)
    let first: i8 = 1;
    let second: u128 = 10_000;

    println!("i8 - Signed 8 bit integer: {}", first);
    println!("u128 - Unsigned 128 bit integer: {}", second);

    // ! Floating point type
    // ? Contains two types : f32 and f64 and all floating point types are signed
    let float_first: f32 = 1.0;
    let float_second: f64 = 2.0;

    println!("f32 - 32 bit floating point: {}", float_first);
    println!("f64 - 64 bit floating point: {}", float_second);

    // ! Boolean type
    // ? Contains two types : true and false
    let bool: bool = true;
    
    println!("Value of bool: {}", bool);

    // ! Character type
    
    let character: char = 'a';
    let heart_eyed_emoji: char = '😍';

    println!("Value of character: {}", character);
    println!("Heart eyed emoji: {}", heart_eyed_emoji);
}
