#[derive(Debug)]
pub struct Duration { s: u64 }

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {s: s}
    }
}


pub trait Planet {
    fn years_during(&self, d: &Duration) -> f64 {
        d.s as f64 / self.period()
    }

    fn period(&self) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEAR : f64 = 31557600.0;

impl Planet for Mercury {
    fn period(&self) -> f64 {
        EARTH_YEAR * 0.2408467
    }
}

impl Planet for Venus {
    fn period(&self) -> f64 {
        EARTH_YEAR * 0.61519726
    }
}

impl Planet for Earth {
    fn period(&self) -> f64 {
        EARTH_YEAR
    }
}

impl Planet for Mars {
    fn period(&self) -> f64 {
        EARTH_YEAR * 1.8808158
    }
}

impl Planet for Jupiter {
    fn period(&self) -> f64 {
        EARTH_YEAR * 11.862615
    }
}

impl Planet for Saturn {
    fn period(&self) -> f64 {
        EARTH_YEAR * 29.447498
    }
}

impl Planet for Uranus {
    fn period(&self) -> f64 {
        EARTH_YEAR * 84.016846
    }
}
impl Planet for Neptune {
    fn period(&self) -> f64 {
        EARTH_YEAR * 164.79132
    }
}

fn main(){
    let mut user_age = String::from("");

    println!("Enter your age: ");
    std::io::stdin().read_line(&mut user_age).unwrap();
    let user_age: u64 = user_age.trim().parse().unwrap();
    let duration = age_to_seconds(user_age);

    println!("Your age on Mercury: {:.2} years", Mercury.years_during(&duration));
    println!("Your age on Venus: {:.2} years", Venus.years_during(&duration));
    println!("Your age on Earth: {:.2} years", Earth.years_during(&duration));
    println!("Your age on Mars: {:.2} years", Mars.years_during(&duration));
    println!("Your age on Jupiter: {:.2} years", Jupiter.years_during(&duration));
    println!("Your age on Saturn: {:.2} years", Saturn.years_during(&duration));
    println!("Your age on Uranus: {:.2} years", Uranus.years_during(&duration));
    println!("Your age on Neptune: {:.2} years", Neptune.years_during(&duration));

}

fn age_to_seconds(age: u64) -> Duration {
    Duration::from(age * 31557600)
}

