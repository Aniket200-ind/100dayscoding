
pub mod letters {
    pub fn count_letters(text: &str) -> usize {
        text.chars().filter(|ref c| c.is_alphabetic()).count()
    }
}

pub mod numbers {
    pub fn count_numbers(text: &str) -> usize {
        text.chars().filter(|ref c| c.is_numeric()).count()
    }
}