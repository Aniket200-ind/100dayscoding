mod text_processing;

fn main() {
    let text = "lorem ipsum 123 dolor sit amet";
    let (number_of_letters, number_of_numbers) = count_letters_and_numbers(text);
    println!("Number of letters: {}", number_of_letters);
    println!("Number of numbers: {}", number_of_numbers);   
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}
