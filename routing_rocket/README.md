# Routing in Rocket

## What is routing in Web Development?

- Routing is the process of creating a URL endpoint (URL pattern) that is associated with a specific resource. When a URL is requested, the server decides which resource to return based on the URL.

- In web development, routing is the process of using URLs to drive the user interface and display the appropriate content to the user.

- In simple terms, routing is the process of determining what content to display based on the URL.


## Routing in Rocket

- In Rocket, routing is the process of associating a URL pattern with a specific resource.

- In Rocket, routing is done using the `#[get("/")]` attribute. This attribute is used to associate a URL pattern with a specific resource.

- Functions that are associated with a URL pattern using the `#[get("/")]` attribute are called route handlers.

- Route handlers are functions that are called when a URL pattern is requested.

- In Rocket, route handlers are used to handle HTTP requests and return the appropriate response.

- In Rocket, route handlers are used to handle GET, POST, PUT, DELETE, and other HTTP requests.

```rust
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
fn about() -> &'static str {
    "About page"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, about])
        .launch();
}
```

- In the above example, we have defined two route handlers `index` and `about` using the `#[get("/")]` attribute.

- The `index` route handler is associated with the URL pattern `/` and the `about` route handler is associated with the URL pattern `/about`.

- The `index` route handler returns the string `Hello, world!` and the `about` route handler returns the string `About page`.


### Dynamic Routing

- In Rocket, dynamic routing is the process of using URL parameters to create dynamic URL patterns.

- In Rocket, dynamic routing is done using the `#[get("/<name>")]` attribute. This attribute is used to create a dynamic URL pattern with a URL parameter.

- URL parameters are used to create dynamic URL patterns that can be used to pass data to the route handler.

- In Rocket, URL parameters are defined using the `<name>` syntax. The name of the URL parameter is enclosed in angle brackets `<` and `>`.

```rust
#[get("/users/<name>")]
fn user(name: &str) -> String {
    format!("Hello, {}", name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![user])
        .launch();
}
```

- In the above example, we have defined a route handler `user` using the `#[get("/users/<name>")]` attribute.

- The `user` route handler is associated with the URL pattern `/users/<name>`. The `<name>` URL parameter is used to create a dynamic URL pattern.

- The `user` route handler takes a URL parameter `name` of type `&str` and returns a string `Hello, <name>`.

### Catch all routes

- In Rocket, catch all routes are used to handle requests for all URL patterns that do not match any other route handlers.

- In Rocket, catch all routes are defined using the `#[catch(404)]` attribute. This attribute is used to define a catch all route handler for handling requests for all URL patterns that do not match any other route handlers.

- Catch all route handlers are used to handle requests for all URL patterns that do not match any other route handlers.

```rust
#[catch(404)]
fn not_found() -> &'static str {
    "Whoops! Something went wrong!"
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .launch();
}
```

- In the above example, we have defined a catch all route handler `not_found` using the `#[catch(404)]` attribute.

- The `not_found` route handler is used to handle requests for all URL patterns that do not match any other route handlers.

- The `not_found` route handler returns the string `Whoops! Something went wrong!`.