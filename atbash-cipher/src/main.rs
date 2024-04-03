fn main() {
    let mut text = String::new();

    println!("Welcome to Atbash cipher");
    println!("Enter text to encode/decode:");
    std::io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();

    let cipher = encode(text);
    println!("Encoded text: {}", cipher);

    let decoded = decode(&cipher);
    println!("Decoded text: {}", decoded);

}

fn decode(plain: &str) -> String {
    let mut cipher = String::new();
    for c in plain.chars() {
        cipher.push(encode_decode_char(c));
    }
    cipher
}

fn encode(plain: &str) -> String {
    let mut cipher = String::new();
    cipher.reserve(plain.len()); // Pre-allocate memory
    for c in plain.chars() {
        cipher.push(encode_decode_char(c));
    }
    cipher.trim().to_string()
}

fn encode_decode_char(c: char) -> char {
    if c.is_alphabetic() {
        let base = if c.is_lowercase() { 'a' } else { 'A' };
        let offset = c as u8 - base as u8;
        let encoded = (25 - offset + base as u8) as char;
        encoded
    } else {
        c
    }
}