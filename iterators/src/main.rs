struct Counter {
    count: usize,
    max: usize,
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let numbers = [2, 1, 17, 99, 34, 56];
    let numbers_iterator = numbers.iter();
    for number in numbers_iterator {
        println!("{}", number);
    }

    // Usage of the custom iterator
    let mut counter = Counter { count: 0, max: 5 };
    while let Some(x) = counter.next() {
        println!("{}", x);
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared numbers: {:?}", squared_numbers);
}
