fn main() {
    {
        let s = String::from("hello"); // s is the owner of the string "hello"
    } // s goes out of scope here, and the memory is freed


    let x = String::from("hello");
    let y = x; // Ownership of the string is moved to y
    // x is no longer valid here


    let s = String::from("hello");
    let len = calculate_length(&s); // s is borrowed by calculate_length
    println!("The length of '{}' is {}.", s, len);

    let mut string_val = String::new();

    change(&mut string_val);
    println!("{string_val}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

fn change(string: &mut String) {
    string.push_str("Hello Rustaceans");
}