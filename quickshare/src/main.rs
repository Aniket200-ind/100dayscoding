use actix_web::{App, get, web, HttpResponse, HttpServer, Responder};
// use rusqlite::{params, Connection};
// use rand::distributions::Alphanumeric;
// use rand::{thread_rng, Rng};
// use std::sync::Mutex;
// use actix_files::NamedFile;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://localhost:8000/");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("localhost:8000")?
    .run()
    .await
}
