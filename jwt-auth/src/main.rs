mod auth;
mod error;

use serde::{Deserialize, Serialize};

enum Role {
    Admin,
    User,
}

pub struct User {
    pub uid: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}