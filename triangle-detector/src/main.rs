use std::ops::Add;
pub struct Triangle<T> {
    sides: [T; 3],
}
impl<T> Triangle<T>
where
    T: Copy + PartialOrd + Add<Output = T> + From<i32>,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        if !sides.iter().all(|s| s > &T::from(0)) {
            return None;
        }
        let mut sides = sides;
        sides.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        if sides[0] + sides[1] > sides[2] {
            Some(Self { sides })
        } else {
            None
        }
    }
    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[2]
    }
    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2]
    }
    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}

fn main(){
    let triangle = Triangle::build([3, 4, 5]);
    match triangle {
        Some(triangle) => {
            println!("Triangle is equilateral: {}", triangle.is_equilateral());
            println!("Triangle is scalene: {}", triangle.is_scalene());
            println!("Triangle is isosceles: {}", triangle.is_isosceles());
        }
        None => {
            println!("Triangle is not valid");
        }
    }
}