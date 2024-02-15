use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Vec<WeatherData>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
}