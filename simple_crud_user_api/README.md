# Simple Crud User API with Actix-Web and SurrealDB

This is a simple CRUD API for storing user data. It uses Actix-Web as the web framework and SurrealDB as the database.

While making this API, I learned how to use Actix-Web and SurrealDB. I also learned how to use the `uuid` crate to generate unique identifiers for the users. This small project helped me understand how to use Actix-Web to create a REST API and also help to strengthen my understanding of Rust.

## Requirements

- Rust
- SurrealDB

## Running the API

1. Clone the repository
2. Run `surrealdb start file:users.db` at the root of the repository to start SurrealDB
3. Run `cargo run` at the root of the repository to start the API
4. The API will be available at `http://localhost:8080`
5. You can use postman or any other tool to test the API

## Endpoints

- `GET /users`: To get all users

- `POST /create_user`: To create a user
  - Body: `{"name": "John Doe", "age": 25, "email": "abc@gmail.com"}`

- `PUT /update_user/{uuid}`: To update a user
  - Body: include all the fields and their new values that you want to update

- `DELETE /delete_user/{uuid}`: To delete a user


