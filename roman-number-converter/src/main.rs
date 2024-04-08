use std::fmt::{Display, Formatter, Result};

struct Roman{
    value: u32,
    symbol: String,
}

impl Display for Roman{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "{}", self.symbol)
    }
}

impl From<u32> for Roman{
    fn from(value: u32) -> Self {
        let roman_numerals = [
            Roman { value: 1000, symbol: "M".to_string() },
            Roman { value: 900, symbol: "CM".to_string() },
            Roman { value: 500, symbol: "D".to_string() },
            Roman { value: 400, symbol: "CD".to_string() },
            Roman { value: 100, symbol: "C".to_string() },
            Roman { value: 90, symbol: "XC".to_string() },
            Roman { value: 50, symbol: "L".to_string() },
            Roman { value: 40, symbol: "XL".to_string() },
            Roman { value: 10, symbol: "X".to_string() },
            Roman { value: 9, symbol: "IX".to_string() },
            Roman { value: 5, symbol: "V".to_string() },
            Roman { value: 4, symbol: "IV".to_string() },
            Roman { value: 1, symbol: "I".to_string() },
        ];
        let mut value = value;
        let mut result = String::new();
        for numeral in roman_numerals.iter() {
            while value >= numeral.value {
                result.push_str(&numeral.symbol);
                value -= numeral.value;
            }
        }
        Roman { value, symbol: result }
    }
}

fn main() {
    let mut user_input = String::new();
    println!("Enter an Arabic number: ");
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_input: u32 = user_input.trim().parse().unwrap();
    let roman: Roman = user_input.into();
    println!("Roman numeral: {}", roman);
}
