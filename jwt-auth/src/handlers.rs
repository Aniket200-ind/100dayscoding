use crate::{auth, error};
use crate::models::{LoginRequest, LoginResponse, User, Role};
use std::convert::Infallible;
use warp::{Filter, reject, Rejection, Reply, reply};
use std::sync::Arc;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;


pub fn with_users(user: User) -> impl Filter<Extract = (User,), Error = Infallible> + Clone {
    warp::any().map(move || user.clone())
}

pub async fn login_handler(users: User, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.password == body.password)
    {
        Some((uid, _user)) => {

        }
        None => Err(reject::custom(error::Error::Unauthorized)),
    }
}

pub fn init_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "1".to_string(),
        User {
            uid: "1".to_string(),
            email: "johndoe@gmail.com".to_string(),
            password: "password".to_string(),
            role: Role::User,
        },
    );
    users.insert(
        "2".to_string(),
        User {
            uid: "2".to_string(),
            email: "admin@yahoo.com".to_string(),
            password: "admin".to_string(),
            role: Role::Admin,
        },
    );

    users
}
