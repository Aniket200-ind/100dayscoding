fn main() {
    let mut input_string = String::new();
    println!("Enter a string to reverse it: ");
    std::io::stdin().read_line(&mut input_string).unwrap();
    let reversed_string = reverse(&input_string);
    println!("Reversed string: {}", reversed_string);
}

fn reverse(input: &str) -> String{
    input.chars().rev().collect()
}
