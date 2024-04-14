use capitalize::Capitalize;
use std::io;

struct Car {
    color: String,
    car_type: String,
    transmission: Transmission,
    convertible: bool,
    brand: String,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn main() {
    let mut car_color = String::new();
    let mut car_transmission = String::new();
    let mut car_brand = String::new();
    let mut convertible = String::new();
    let mut car_type = String::new();

    println!("Enter the car color:");
    take_user_input(&mut car_color);

    println!("Enter the car transmission: (Manual, SemiAuto or Automatic))");
    take_user_input(&mut car_transmission);

    println!("Enter the car brand:");
    take_user_input(&mut car_brand);

    println!("Do you want to have a convertible? (yes or no)");
    take_user_input(&mut convertible);

    println!("Enter the car type:");
    take_user_input(&mut car_type);

    let car_color = car_color.trim().to_string().capitalize();
    let car_transmission = car_transmission.trim().to_lowercase();
    let car_brand = car_brand.trim().to_string();
    let convertible = convertible.trim().to_lowercase();
    let car_type = car_type.trim().to_string().capitalize();

    let car_transmission = match car_transmission.as_str() {
        "manual" => Transmission::Manual,
        "semiAuto" => Transmission::SemiAuto,
        "automatic" => Transmission::Automatic,
        _ => panic!("The car can only be one of the three options: Manual, SemiAuto or Automatic"),
    };

    let convertible = match convertible.as_str() {
        "yes" => true,
        "no" => false,
        _ => panic!("The car can either be true or false"),
    };

    let car = car_factory(
        car_color,
        car_transmission,
        convertible,
        car_brand,
        car_type,
    );
    println!("Your car has been built with the following specifications:");
    println!(
        "Car brand: {}, Type: {}, Color: {}, Transmission: {:?}, Convertible: {}",
        car.brand, car.car_type, car.color, car.transmission, car.convertible
    );
}

fn car_factory(
    color: String,
    transmission: Transmission,
    convertible: bool,
    brand: String,
    car_type: String,
) -> Car {
    // All new cars always have zero mileage
    Car {
        color,
        transmission,
        convertible,
        brand,
        car_type,
    }
}

fn take_user_input(user_input: &mut String) -> String {
    io::stdin()
        .read_line(user_input)
        .expect("Failed to read the input");

    user_input.to_string()
}
