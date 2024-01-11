#[derive(Debug)]
struct Student {
    id: u8,
    name: String,
    roll_no: u8,
    course_names: [String; 3],
}

fn main() {
    let stud1 = Student {
        id: 1,
        name: String::from("Aniket"),
        roll_no: 9,
        course_names: [
            String::from("JavaScript"),
            String::from("Rust"),
            String::from("Full Stack Web Development"),
        ],
    };

    println!(
        "Student {} has registered for courses {}, namely {:?}. His roll no. is {} and id is {}",
        stud1.name,
        stud1.course_names.len(),
        stud1.course_names,
        stud1.roll_no,
        stud1.id,
    );

    let mut stud2 = Student {
        id: 2,
        name: String::from("John Doe"),
        roll_no: 20,
        ..stud1
    };

    stud2.roll_no = 5;

    println!("{:?}", stud2);
    // ! For above line to work we need to add the Debug trait to the struct
}
