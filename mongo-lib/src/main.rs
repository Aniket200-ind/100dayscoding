mod handlers;
mod error;
mod db;

use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply, http::StatusCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book{
    pub id: String,
    pub title: String,
    pub author: String,
    pub num_pages: usize,
    pub price: f64,
    pub in_stock: bool,
    pub tags: Vec<String>,
    pub added_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {

}

