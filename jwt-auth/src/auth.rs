use crate::models::{Role};
use chrono::prelude::*;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::fmt;
use warp::{filters::header::headers_cloned, http::header::{HeaderMap, HeaderValue, AUTHORIZATION}, reject, Filter, Rejection, header};

const BEARER: &str = "Bearer";
const JWT_SECRET: &[u8] = b"secret";

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims{
    sub: String,
    role: String,
    exp: usize,
}

pub fn with_auth(role: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

pub fn create_jwt(uid: &str, role: &Role) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims  {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)
        .map_err(|_| Error::JWTTokenCreationError)
    )
}