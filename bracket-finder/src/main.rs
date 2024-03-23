fn main() {
    let input_string = "({[Hello WOrld]Rust})";
    let result = is_matched(input_string);
    println!("Are all the brackets in the input string matched? {}", result);
}

fn is_matched(input_string: &str) -> bool {
    let mut stack = Vec::new();

    for ch in input_string.chars(){
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => (),
        }
    }
    stack.is_empty()
}