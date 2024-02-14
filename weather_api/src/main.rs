use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use reqwest::Client;
use std::env;

mod models;
use crate::models::Weather;

#[get("/")]
async fn index() -> impl Responder {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let city = "Mumbai";

    todo!("Implement a another api for AQI => https://openweathermap.org/api/air-pollution");

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
    let response = Client::new().get(&url).send().await.unwrap();
    let body = response.text().await.unwrap();

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