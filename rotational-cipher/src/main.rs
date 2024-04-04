fn main() {
    println!("Hello, world!");
    todo!("Update the cipher implementation");
    let input = "Hello, World!";
    let key = 5;
    let encrypted = rotate(input, key);
    println!("Encrypted: {}", encrypted);
}

fn rotate(input: &str, key: u8) -> String {
    // give logic to apply rotation cipher and it should be performant and fast use optimized code
    let mut result = String::new();
    for c in input.chars() {
        let mut new_char = c as u8;
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            new_char = (new_char - base + key) % 26 + base;
        }
        result.push(new_char as char);
    }
    result
}
