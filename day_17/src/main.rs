use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();
    marks.insert("Web Development", 95); 
    marks.insert("Python", 76); 
    marks.insert("Rust", 80); 
    marks.insert("DBMS", 70); 

    println!("Subjects studied are {:?}", marks.keys());

    if marks.contains_key(& "Python") {
        println!("Student has studied Python");
    }else{
        println!("Student has not studied Python");
    }

    for (subject, mark) in marks{
        println!("Student has scored {} marks in {} subject", subject, mark);
    }

}
