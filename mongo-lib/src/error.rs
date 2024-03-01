use serde::{Serialize, Deserialize};
use thiserror::Error;
use warp::{http::StatusCode, reply, Rejection, Reply};
use std::convert::Infallible;
use mongodb::bson;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("MongoDB query error: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("Could not access field in the document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("Invalid ID error: {0}")]
    InvalidIDError(String)
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<Box<dyn Reply>, Infallible> {
    let code: StatusCode;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found".to_string();
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body".to_string();
    } else if let Some(e) = err.find::<Error>(){
        match e{
            _=> {
                eprintln!("Unhandled application error: {:?}", e);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error".to_string();
            }
        }
    }
    else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed".to_string();
    }else{
        eprintln!("Unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error".to_string();
    }
    let json = reply::json(&ErrorResponse { message: message.into() });
    Ok(Box::new(reply::with_status(json, code)))
}