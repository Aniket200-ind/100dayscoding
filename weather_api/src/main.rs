use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080/");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("localhost:8080")?
    .run()
    .await
}