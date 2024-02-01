macro_rules! vec {
    ($($x:expr), *) => {
        {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )* // * is the repetition operator
        temp_vec
    }
};
}

fn main() {
    let v:Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", v);
}
