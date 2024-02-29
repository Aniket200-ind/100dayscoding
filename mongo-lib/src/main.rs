mod handlers;
mod error;
mod db;
mod model;

use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply, http::StatusCode};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;


#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;
    let books = warp::path("books");

    let book_routes = books
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_book_handler)
        .or(books
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::update_book_handler)
        )
        .or(books
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_book_handler)
        )
        .or(books
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::books_list_handler)
        );

    let routes = book_routes.recover(error::handle_rejection);
    println!("Starting server at http://localhost:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

