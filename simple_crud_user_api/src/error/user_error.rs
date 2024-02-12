use derive_more::Display;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use actix_web::body::BoxBody;

#[derive(Debug, Display)]
pub enum UserError{
    NoUsersFound,
    UserAlreadyExists,
    UserCreationFailed,
    UserUpdateFailed,
    UserDeletionFailed,
    UserRetrievalFailed,

}

impl ResponseError for UserError{
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::NoUsersFound => StatusCode::NOT_FOUND,
            UserError::UserAlreadyExists => StatusCode::BAD_REQUEST,
            UserError::UserCreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserUpdateFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserDeletionFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserRetrievalFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}