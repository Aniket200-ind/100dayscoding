use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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