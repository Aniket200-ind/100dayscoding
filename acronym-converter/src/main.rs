fn main() {
    let mut user_input = String::new();
    println!("Enter a phrase to generate its acronym: ");
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim().to_string();
    let acronym = abbreviate(&user_input);
    println!("The acronym for '{}' is '{}'", user_input, acronym);
}

fn abbreviate(user_input: &String) -> String {
    user_input
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}