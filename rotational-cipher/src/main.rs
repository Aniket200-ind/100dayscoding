fn main() {
    let input = read_input("Enter a message to encrypt:");
    let key: i8 = read_input("Enter a key:").parse().expect("Key must be a number");
    let encrypted = rotate(input, key);
    println!("Encrypted: {}", encrypted);
}
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.trim().to_string()
}
pub fn rotate(input: String, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => rot(c as u8, b'a', key),
            'A'..='Z' => rot(c as u8, b'A', key),
            _ => c,
        })
        .collect()
}
fn rot(c: u8, lower_bound: u8, key: i8) -> char {
    let new = (c - lower_bound) as i8 + key;
    if new < 0 {
        ((new + 26) as u8 + lower_bound) as char
    } else {
        (new as u8 % 26 + lower_bound) as char
    }
}