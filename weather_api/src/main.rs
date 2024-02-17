use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use std::env;
mod models;
use models::Weather;
use models::AirQualityIndex;
use models::aqi::AirQuality;
use crate::models::weather::WeatherResponse;



#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rustaceans!")
    //Todo: Initialize a readme.md file for project
    // Todo: Learn docker and integrate frontend for this project in React
}

#[get("/api/weather-data")]
async fn weather_data() -> impl Responder {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let city = "Satara";

    // Weather API
    let weather_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let weather_response = Client::new().get(&weather_url).send().await.expect("Failed to send request");
    let weather_data: Weather = weather_response.json().await.expect("Failed to parse JSON response");

    let lat = weather_data.coord.lat;
    let lon = weather_data.coord.lon;
    let temperture = weather_data.main.temp;
    let description = &weather_data.weather[0].description;
    let min_temp = weather_data.main.temp_min;
    let feels_like = weather_data.main.feels_like;
    let max_temp = weather_data.main.temp_max;
    let humidity = weather_data.main.humidity;
    let wind_speed = weather_data.wind.speed;
    let city_name = &weather_data.name;

    let aqi_url = format!(
        "https://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );
    let aqi_response = Client::new().get(&aqi_url).send().await.expect("Failed to send request");
    let aqi: AirQualityIndex = aqi_response.json().await.expect("Failed to parse JSON response");

    let air_quality_index = aqi.list[0].main.aqi;
    let co = aqi.list[0].components.co;
    let no = aqi.list[0].components.no;
    let no2 = aqi.list[0].components.no2;
    let o3 = aqi.list[0].components.o3;
    let so2 = aqi.list[0].components.so2;
    let pm2_5 = aqi.list[0].components.pm2_5;
    let pm10 = aqi.list[0].components.pm10;
    let nh3 = aqi.list[0].components.nh3;



    let air_quality = match air_quality_index{
        1 => AirQuality::Good,
        2 => AirQuality::Fair,
        3 => AirQuality::Moderate,
        4 => AirQuality::Poor,
        5 => AirQuality::VeryPoor,
        _ => AirQuality::Good,
    };

    HttpResponse::Ok().json(WeatherResponse{
        lat,
        lon,
        temperture,
        description: description.to_string(),
        min_temp,
        feels_like,
        max_temp,
        humidity,
        wind_speed,
        city_name: city_name.to_string(),
        air_quality_index,
        air_quality,
        co,
        no,
        no2,
        o3,
        so2,
        pm2_5,
        pm10,
        nh3
    })
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Server running at http://localhost:8080/");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(weather_data)
    })
        .bind("localhost:8080")?
        .run()
        .await
}
