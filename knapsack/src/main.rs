#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = vec![0; (max_weight + 1) as usize];
    for item in items {
        for weight in (item.weight..=max_weight).rev() {
            dp[weight as usize] = dp[weight as usize].max(dp[(weight - item.weight) as usize] + item.value);
        }
    }
    dp[max_weight as usize]
}



fn main() {
    let items = [
        Item { weight: 1, value: 1 },
        Item { weight: 3, value: 4 },
        Item { weight: 4, value: 5 },
        Item { weight: 5, value: 7 },
    ];
    let max_weight = 7;
    let result = maximum_value(max_weight, &items);
    println!("Maximum value: {}", result);
}
