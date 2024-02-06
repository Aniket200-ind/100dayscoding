#[macro_use]
extern crate rocket;

use rocket::response::content::RawJson;
use rocket::Request;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    first_name: String,
    last_name: String,
}

#[get("/hello")]
fn hello() -> RawJson<&'static str> {
    RawJson(
        r#"
        {
            "status": "Sucess",
            "message": "Hello, API",
        }
    "#,
    )
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!(
        "Oh no🥲! We couldn't find the requested path '{}'",
        req.uri()
    )
}

#[get("/")]
fn home_page() -> Template {
    let context = User {
        first_name: "Aniket".to_string(),
        last_name: "Botre".to_string(),
    };
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home_page, hello])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
