use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use std::env;
mod models;
use models::Weather;
use models::AirQualityIndex;
use models::aqi::AirQuality;


#[get("/")]
async fn index() -> impl Responder {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let city = "Mumbai";

    // Weather API
    let weather_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let weather_response = Client::new().get(&weather_url).send().await.unwrap();
todo!("Parse the JSON response into a Weather struct and return it as json response. Also make sure to use lat and lon from the Weather struct to call the Air Quality Index API.");
    let body = weather_response.text().await.unwrap();

    // Air Quality Index API
    // let aqi_url = format!(
    //     "https://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
    //     lat, lon, api_key
    // );
    // let aqi_response = Client::new().get(&url).send().await.unwrap();
    // let body = response.text().await.unwrap();

    HttpResponse::Ok().body(body)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Server running at http://localhost:8080/");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("localhost:8080")?
        .run()
        .await
}
