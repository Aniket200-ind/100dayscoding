use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("This is simple server built with Actix-web")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hello", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
