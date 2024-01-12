fn main() {
    let s1 = "Hello World!";
    println!("{s1} created with string literals");

    let s2 = String::from("Hello World!");
    println!("{s2} created with String object");

    let mut dynamic_str = String::from("Hello, ");
    dynamic_str.push_str("Rustaceans!");
    println!("{}", dynamic_str);

    let mut user_input = String::new();
    user_input.push_str("Username: Aniket");
    println!("{user_input}");

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");
    let game_str = format!("{str1} - {str2} - {str3}");
    println!("{game_str}");
}
