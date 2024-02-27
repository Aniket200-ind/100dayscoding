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
}

