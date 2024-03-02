use crate::{error::Error::*, handlers::BookRequest, model::Book, Result};

use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{document::Document, oid::ObjectId, doc};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "bookstore";
const COLLECTION_NAME: &str = "books";
const ID: &str = "_id";
const TITLE: &str = "title";
const AUTHOR: &str = "author";
const NUM_PAGES: &str = "num_pages";
const PRICE: &str = "price";
const IN_STOCK: &str = "in_stock";
const TAGS: &str = "tags";
const ADDED_AT: &str = "added_at";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb+srv://aniketbotre007:futureisJavaScript@rustbookapi.t6nffhu.mongodb.net/?retryWrites=true&w=majority&appName=RustBookAPI").await?;
        client_options.app_name = Some("bookstore".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>> {
        let mut cursor = self
            .get_collection()
            .await
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_book(doc?)?);
        }
        Ok(result)
    }

    pub async fn get_collection(&self) -> Collection<Book> {
        self.client.database(DB_NAME).collection(COLLECTION_NAME)
    }

    pub async fn doc_to_book(&self, doc: Document) -> Result<Book> {
        Ok(Book {
            id: doc.get_object_id(ID)?.to_hex(),
            title: doc.get_str(TITLE)?.to_string(),
            author: doc.get_str(AUTHOR)?.to_string(),
            num_pages: doc.get_i32(NUM_PAGES)? as usize,
            price: doc.get_f64(PRICE)?,
            in_stock: doc.get_bool(IN_STOCK)?,
            tags: doc.get_array(TAGS)?.iter().map(|entry| entry.to_string()).collect(),
            added_at: doc.get_datetime(ADDED_AT)?.into(),
        })
    }

    pub async fn create_book(&self, book: &BookRequest) -> Result<()> {
        let doc = doc! {
            TITLE: &book.title,
            AUTHOR: &book.author,
            NUM_PAGES: book.num_pages as i32,
            PRICE: book.price,
            IN_STOCK: book.in_stock,
            TAGS: &book.tags,
            ADDED_AT: Utc::now(),
        };
        self.get_collection().insert_one(doc, None).await.map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn update_book(&self, id: String, book: &BookRequest) -> Result<()> {
        let object_id = ObjectId::with_string(&id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {"_id": object_id};
        let update = doc! {
            TITLE: &book.title,
            AUTHOR: &book.author,
            NUM_PAGES: book.num_pages as i32,
            PRICE: book.price,
            IN_STOCK: book.in_stock,
            TAGS: &book.tags,
            ADDED_AT: Utc::now()
        };
        self.get_collection().await.update_one(filter, update, None).await.map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_book(&self, id: String) -> Result<()> {
        let object_id = ObjectId::from(id.clone());
        let filter = doc! {"_id": object_id};
        self.get_collection().await.delete_one(filter, None).await.map_err(MongoQueryError)?;
        Ok(())
    }
}