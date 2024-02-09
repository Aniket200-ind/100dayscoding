use actix_web::{get, web, App, HttpServer};
use std::sync::Mutex;

struct Appstate {
    app_name: String,
    counter: Mutex<u32>,
}

#[get("/")]
async fn index(data: web::Data<Appstate>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {} for {} website", counter, app_name)
}

#[get("/about")]
async fn about(data: web::Data<Appstate>) -> String {
    let app_name = &data.app_name;
    format!("This is about page of {} website", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Appstate {
                app_name: String::from("Managing State in Actix-web"),
                counter: Mutex::new(0),
            }))
            .service(index)
            .service(about)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
