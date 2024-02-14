use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Weather{
    coord: Coord,
    weather: Vec<WeatherData>,
    main: Main,
    wind: Wind,
    name: String
}

#[derive(Deserialize)]
pub struct Coord{
    lon: f32,
    lat: f32
}

#[derive(Deserialize)]
pub struct WeatherData{
    main: String,
    description: String
}

#[derive(Deserialize)]
pub struct Main{
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    humidity: i32
}

#[derive(Deserialize)]
pub struct Wind{
    speed: f32,
}
