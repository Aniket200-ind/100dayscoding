#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    let h_map = hashmap!{
        "key1" => "value1",
        "key2" => "value2",
        "key3" => "value3",
    };
    println!("{:?}", h_map);
}