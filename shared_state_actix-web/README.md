# Shrared state and mutability using Actix Web

Actix Web provides a way to share state across multiple handlers. This is useful when you want to share a database connection pool or some other resource across multiple handlers. You can do this by using the `App::data` method to register the shared state.


## Shared state management

In Actix Web, shared state is managed using the `Data` type. The `Data` type is a container that holds a reference to the shared state. You can create a `Data` instance using the `Data::new` method, and then use the `App::data` method to register the shared state with the application.

```rust
use actix_web::{web, App, HttpServer};

struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // get app_name
    format!("Hello {}!", app_name) // format response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080",)?
    .run()
    .await
}
```

In this example, we define a `AppState` struct to hold the shared state. We then use the `App::data` method to register the shared state with the application. In the `index` handler, we use the `web::Data` extractor to access the shared state. The `web::Data` extractor is a special extractor that allows you to access the shared state from within a handler. The `web::Data` extractor takes a type parameter that specifies the type of the shared state. In this case, the type parameter is `AppState`, which is the type of the shared state.


## Mutability

You can also use the `App::app_data` method to register mutable shared state with the application. This is useful when you want to share a mutable resource across multiple handlers. You can create a `App::app_data` instance using the `App::app_data` method, and then use the `App::app_data` method to register the mutable shared state with the application. We need to create a `Mutex` for the mutable shared state. The mutex is used to ensure that only one handler can access the mutable shared state at a time.

```rust
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    format!("Request number: {}", counter) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(AppStateWithCounter {
                counter: Mutex::new(0),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

In this example, we define a `AppStateWithCounter` struct to hold the mutable shared state. We then use the `App::app_data` method to register the mutable shared state with the application. In the `index` handler, we use the `web::Data` extractor to access the mutable shared state. The `web::Data` extractor is a special extractor that allows you to access the shared state from within a handler. The `web::Data` extractor takes a type parameter that specifies the type of the shared state. In this case, the type parameter is `AppStateWithCounter`, which is the type of the shared state.

The `Mutex` type is used to ensure that only one handler can access the mutable shared state at a time. The `Mutex` type provides a way to safely mutate the shared state across multiple handlers. The `Mutex` type is a synchronization primitive that allows you to lock and unlock the shared state to ensure that only one handler can access the mutable shared state at a time.


## Conclusion

In this article, we discussed how to share state and manage mutability in Actix Web. We learned how to use the `App::data` method to register shared state with the application, and how to use the `web::Data` extractor to access the shared state from within a handler. We also learned how to use the `App::app_data` method to register mutable shared state with the application, and how to use the `Mutex` type to ensure that only one handler can access the mutable shared state at a time. By using these techniques, you can share state and manage mutability in Actix Web to build powerful and scalable web applications.