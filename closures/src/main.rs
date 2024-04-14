fn main() {
    let outer_var = 10;
    let result = | x: i32 | x + outer_var;
    let greeting = || println!("Hello, Rustaceans!");
    println!("10 + 10 = {}", result(10));
    println!("10 + 2 = {}", result(2));
    greeting();

    let str = "Hello,";

    let concat_str = | val: &str | format!("{} {}", str, val);
    println!("{}", concat_str("World!"));
    println!("{}", concat_str("Developers!"));

    let greet = | name: &str , time_of_the_day: &str | format!("Good {} {}!", time_of_the_day, name);
    println!("{}", greet("Aniket", "Evening"));

    let mut list = vec![1, 2, 3, 4, 5];

    let mut add_num = || list.push(10);

    // println!("List before adding the list: {:?}", list);

    add_num();

    println!("List: {:?}", list);

}
