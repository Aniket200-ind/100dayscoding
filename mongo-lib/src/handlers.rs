use crate::{WebResult, db::DB};
use serde::{Serialize, Deserialize};
use warp::{http::StatusCode, reply::json, reject, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub author: String,
    pub num_pages: usize,
    pub price: f64,
    pub in_stock: bool,
    pub tags: Vec<String>,
}

pub async fn books_list_handler(db: DB) -> WebResult<impl Reply>{
    let books = db.fetch_books().await.map_err(|e| reject::custom(e))?;
    Ok(json(&books))
}

pub async fn create_book_handler(book: BookRequest, db: DB) -> WebResult<impl Reply>{
    db.create_book(book).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn update_book_handler(id: String, book: BookRequest, db: DB) -> WebResult<impl Reply>{
    db.update_book(id, book).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_book_handler(id: String, db: DB) -> WebResult<impl Reply>{
    db.delete_book(id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::NO_CONTENT)
}