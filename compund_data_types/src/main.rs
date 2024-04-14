fn main() {
    // ! Tuples
    let mut tup = (1, 2.4, 'a', -211, 511);
    println!("Tuple tap {:?}", tup);

    //? Accessing elements
    let first = tup.0;
    tup.1 = 33.5;
    println!("The value of first variable is {first}");

    // ? Tuple destructuring
    let (first_num, float_value, char_value, negative_num, num) = tup;

    println!("The value of first number in tuple is {first_num}, float_value is {float_value}, negative_num is {negative_num}, char's value is 
    {char_value}, and num's value is {num}");

    // ! Array
    let arr1 = [1, 2, 3, 4, 5];
    println!("Values in arr1 are {:?}", arr1);

    let arr2 = [12; 4];

    println!("Values in arr2 are {:?}", arr2);
    
    // ? Array destructuring
    let [x, y, z] = [1, 3, 4];
    println!("Value of x is {x}, y is {y} and z is {z}");   

    //? Accessing array elements
    let first = arr1[0];
    println!("First value in arr1 is {:?}", first);

    println!("The length of array arr1 is {}", arr1.len());
}
