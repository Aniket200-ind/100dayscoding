#[derive(Debug)]
enum Student {
    Name(String),
    RollNo(u8),
}

fn main() {
    let vector = vec![1, 2, 3, 4];
    println!("Vector1: {:?}", vector);

    let arr = [10, 20, 30, 40];

    let vector2 = Vec::from(arr);
    
    for i in &vector2{
        println!("{}", i);
    }

    let mut vector3 = Vec::new();
    vector3.push(22);
    vector3.push(99);
    vector3.push(66);
    vector3.push(44);
    println!("Vector3: {:?}", vector3);
    vector3.pop();

    if !vector3.contains(&44){
    println!("Vector3 after pop: {:?}", vector3);
    }

    // ! While declaring intializing a vector with Vec::capacity(capacity) type annotation is must!!

    //* This initializes a vector with capacity of storing 100 elements */
    let mut vector4: Vec<i32> = Vec::with_capacity(100);

    for i in 101..=150{
        vector4.push(i);
    }

    println!("Length of vector 4 is {}, but it can hold {} elements", vector4.len(), vector4.capacity());

    let student1 = vec!(Student::Name(String::from("Aniket")), Student::RollNo(09));
    let student2 = vec!(Student::Name(String::from("John Doe")), Student::RollNo(14));
    let student3 = vec!(Student::Name(String::from("Harry")), Student::RollNo(10));
    let student4 = vec!(Student::Name(String::from("Harish")), Student::RollNo(29));
    let student5 = vec!(Student::Name(String::from("Ram")), Student::RollNo(19));

    let group = vec![student1, student2, student3, student4, student5];

    println!("Student included in the group are {:?}", group);
}
