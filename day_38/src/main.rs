use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Create route")
}

#[put("/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("Update route")
}

#[delete("/delete")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Delete route")
}

#[get("/users/{name}/{age}")]
async fn user_info(info: web::Path<(String, u8)>) -> impl Responder {
    // * This is also acceptable
    // HttpResponse::Ok().body(format!("User name: {} with age: {}", info.0, info.1))

    let (name, age) = info.into_inner();
    HttpResponse::Ok().body(format!("User name: {} with age: {}", name, age))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(create)
            .service(update)
            .service(delete)
            .service(user_info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
