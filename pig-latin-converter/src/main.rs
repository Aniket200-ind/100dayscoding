fn main() {
    let mut input = String::new();
    println!("Welcome to Pig Latin Translator!");
    println!("Enter a word or sentence to translate to Pig Latin:");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            let output = translate(input);
            println!("The Pig Latin translation is: {}", output);
        },
        Err(e) => eprintln!("Error reading input: {}", e),
    }
}


pub fn translate(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .map(|word| pig(word))
        .collect::<Vec<String>>()
        .join(" ")
}

fn pig(word: &str) -> String {
    let vowels = "aeiou";
    let mut letters = word.chars();
    let first = letters.next().unwrap();
    if vowels.contains(first) || word.starts_with("yt") || word.starts_with("xr") {
        return format!("{}ay", word);
    }
    let mut cluster = String::new();
    cluster.push(first);
    for letter in letters {
        if !vowels.contains(letter) {
            cluster.push(letter);
        } else {
            if cluster.ends_with('q') && letter == 'u' {
                cluster.push(letter);
            }
            break;
        }
    }
    format!(
        "{}{}ay",
        word.chars().skip(cluster.len()).collect::<String>(),
        cluster
    )
}
