use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    loop {
        println!("🧙‍♂️ Welcome to WordWhisperer! 🧙‍♂️");
        println!("1. Count the number of occurrences of word in a file 📄");
        println!("2. Count the number of occurrences of word from user input 💬");
        println!("3. Exit 🚪");

        let mut choice = String::new();
        println!("Enter your choice: ");
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice ❌");
                continue;
            },
        };

        match choice {
            1 => {
                println!("Enter the file name: ");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();
                println!("Enter the word: ");
                let mut word = String::new();
                io::stdin().read_line(&mut word).unwrap();
                let word = word.trim().to_lowercase();
                match count_word_occurence_from_file(&filename, &word) {
                    Ok(count) => println!("The word '{}' occurs {} times in the file '{}' 📚", word, count, filename),
                    Err(e) => println!("Error reading file: {} ❌", e),
                }
            },
            2 => {
                println!("Enter the sentence: ");
                let mut sentence = String::new();
                io::stdin().read_line(&mut sentence).unwrap();
                let sentence = sentence.trim();
                println!("Enter the word: ");
                let mut word = String::new();
                io::stdin().read_line(&mut word).unwrap();
                let word = word.trim().to_lowercase();
                let count = count_word_occurence_from_user_input(&sentence, &word);
                println!("The word '{}' occurs {} times in the sentence ", word, count);
            },
            3 => {
                println!("Exiting the program 🚪");
                println!("Thank you for using WordWhisperer! 🧙‍♂️");
                break;
            },
            _ => {
                println!("Invalid choice ❌");
            }
        }
    }
}

fn count_word_occurence_from_file(file: &str, word: &str) -> io::Result<usize> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut word_count = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        for word_in_line in line.split_whitespace() {
            let word_in_line = word_in_line.to_lowercase();
            if word_in_line == word {
                let count = word_count.entry(word).or_insert(0);
                *count += 1;
            }
        }
    }
    Ok(word_count.get(word).unwrap_or(&0).to_owned())
}

fn count_word_occurence_from_user_input(sentence: &str, word: &str) -> usize {
    let mut word_count = HashMap::new();
    for word_in_sentence in sentence.split_whitespace() {
        let word_in_sentence = word_in_sentence.to_lowercase();
        if word_in_sentence == word {
            let count = word_count.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }
    word_count.get(word).unwrap_or(&0).to_owned()
}