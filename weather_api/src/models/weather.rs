use serde::{Deserialize, Serialize};
use crate::models::aqi::AirQuality;

#[derive(Deserialize, Debug, Serialize)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Vec<WeatherData>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherData {
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Wind {
    pub speed: f32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherResponse {
    pub lat: f32,
    pub lon: f32,
    pub temperture: f32,
    pub description: String,
    pub min_temp: f32,
    pub feels_like: f32,
    pub max_temp: f32,
    pub humidity: i32,
    pub wind_speed: f32,
    pub city_name: String,
    pub air_quality_index: i32,
    pub air_quality: AirQuality,
    pub co: f32,
    pub no: f32,
    pub no2: f32,
    pub o3: f32,
    pub so2: f32,
    pub pm2_5: f32,
    pub pm10: f32,
    pub nh3: f32,
}