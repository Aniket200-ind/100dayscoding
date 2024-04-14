struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    match &person.middle {
        Some(name) => {
            full_name.push_str(&name);
            full_name.push_str(" ");
        }
        None => full_name.push_str(""),
    }

    full_name.push_str(&person.last);

    full_name
}

fn eligible(age: i32) -> Result<i32, String> {
    if age >= 18 {
        return Ok(age);
    } else {
        return Err("Not Eligible..Wait for some years".to_string());
    }
}

fn main() {
    // panic!("Hello world!"); --> This will panic the thread and terminate the program

    let fruits = vec!["mango", "strawberry", "apple", "banana", "orange", "guava"];

    for &index in [0, 1, 2, 3, 4, 5, 10].iter() {
        match fruits.get(index) {
            Some(&"strawberry") => println!("I like strawberries!!"),
            Some(fruit_name) => println!("It's a delicious {}", fruit_name),
            None => println!("Sorry, Fruit not availabe!"),
        }
    }

    let a = Some("dog").unwrap_or("cat");

    println!("{}", a);

    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };

    println!("Your full name is {}", build_full_name(&john));
    println!("Your full name is {}", build_full_name(&alice));
    println!("Your full name is {}", build_full_name(&bob));

    let result = eligible(13);
    match result {
        Ok(age) => {
            println!("Person eligible to vote with age={}", age);
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
