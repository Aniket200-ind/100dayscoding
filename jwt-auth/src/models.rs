use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Clone)]
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