# Rocket framework

## Description

This is a simple framework for creating a web application. It is based on the [Flask](https://flask.palletsprojects.com/en/1.1.x/) framework and uses the [Jinja2](https://jinja.palletsprojects.com/en/2.11.x/) template engine. The framework is designed to be simple and easy to use, and it is suitable for small to medium-sized web applications.

## Hello World

To create a simple "Hello, World!" application, you can use the following code:

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

This code creates a simple web application that listens on the root URL and returns the string "Hello, world!".

## Features

The Rocket framework has the following features:

- Simple and easy to use

- Based on the Flask framework

- Uses the Jinja2 template engine

- Suitable for small to medium-sized web applications

- Supports routing, request handling, and response generation

- Supports static files and templates

- Supports middleware and error handling