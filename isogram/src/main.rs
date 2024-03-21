use std::collections::HashSet;

fn main() {
    let mut word = String::new();
    println!("Enter a word to check if it is an isogram:");
    std::io::stdin().read_line(&mut word).expect("Failed to read line");
    word = word.trim().to_string();
    let is_isogram = check_isogram(&word);
    match is_isogram {
        Some(true) => println!("The word {} is an isogram", word),
        Some(false) => println!("The word {} is not an isogram", word),
        None => println!("The word {} is not an isogram", word),
    }
}

fn check_isogram(word: &String) -> Option<bool>{
    let mut seen_chars = HashSet::new();
    for c in word.to_lowercase().chars(){
        if seen_chars.contains(&c) {
            return Some(false);
        }
        seen_chars.insert(c);
    }
    Some(true)
}
