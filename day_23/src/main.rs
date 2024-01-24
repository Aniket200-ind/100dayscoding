fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(10, 20), 30);
    assert_eq!(add(5, -2), 3);
}


#[test]
#[should_panic]
fn add_fails() {
    assert_eq!(add(2, 2), 5);
    assert_eq!(add(10, 20), 50);
    assert_eq!(add(5, -2), 0);
}

#[cfg(test)]
mod add_function_tests{
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(10, 20), 30);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 5);
        assert_eq!(add(10, 20), 50);
        assert_eq!(add(5, -2), 0);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4);
        assert_eq!(add(-10, -20), -30);
        assert_eq!(add(-5, -2), -7);
    }
}

fn main() {}