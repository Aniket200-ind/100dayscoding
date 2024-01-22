#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);

    let str1 = String::from("Hello, world!");
    let str2 = String::from("Hello, Rustaceans!");

    let result = longest(&str1, &str2);

    println!("The longest string is {}", result);
}
