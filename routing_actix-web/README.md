# Routing using Actix Web

Actix Web provides a powerful, simple, and flexible routing system. It is a system that allows you to define the routes of your application and the actions that will be executed when a request matches a route.

In this module we will cover in short how to define routes using Actix Web.

## Defining Routes

To define routes in Actix Web, you use the `App` and `web` modules. The `App` module is used to configure the application and the `web` module is used to define routes.

Here is an example of how to define routes in Actix Web:

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    Ok(())
}
```

In this example, we define a route that matches the root path `/` and calls the `index` function when a request matches the route. The `index` function returns an `HttpResponse` with a status code of `200` and a body of `Hello world!`.

## Dynamic Parameters

You can also define routes with dynamic parameters. Dynamic parameters are used to capture parts of the URL and pass them as arguments to the handler function.

Here is an example of how to define a route with dynamic parameters:

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

#[get("/users/{user_name}/{user_id}")]
async fn user(info: web::Path<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! Your id is {}", info.0, info.1))
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    Ok(())
}
```

In this example, we define a route that matches the path `/user/{name}/{id}` and calls the `user` function when a request matches the route. The `user` function takes a `web::Path` parameter that captures the `name` and `id` from the URL and passes them as arguments to the function.

## Conclusion

Actix Web provides a powerful and flexible routing system that allows you to define routes and handle requests with ease. In this module, we covered how to define routes and handle dynamic parameters. In the next module, we will cover how to handle different types of requests and responses using Actix Web.
