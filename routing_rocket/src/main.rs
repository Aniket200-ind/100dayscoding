#[macro_use] extern crate rocket;

// * Defining routes
#[get("/")]
fn home() -> &'static str {
    "This is our home page!"
}

#[get("/about")]
fn about() -> &'static str {
    "This is our about page!"
}

#[get("/contact")]
fn contact() -> &'static str {
    "This is our contact page!"
}

// * Dynamic routes
#[get("/users/<name>")]
fn users(name: &str) -> String {
    format!("Hello, {}!", name)
}

// * Catch all route
#[catch(404)]
fn not_found() -> &'static str {
    "Whoops🤖! Page not found!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, about, contact, users])
        .register("/", catchers![not_found])
}
