mod auth;
mod error;
mod models;
mod handlers;

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use warp::{reject, reply, Filter, Rejection, Reply};
use std::collections::HashMap;
use std::convert::Infallible;
use crate::models::{User, LoginRequest, LoginResponse, Role};
use crate::handlers::{login_handler, user_handler, admin_handler, with_auth, handle_rejection, init_users};


#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

