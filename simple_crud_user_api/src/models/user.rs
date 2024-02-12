use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "Name required!!"))]
    pub name: String,
    #[validate(range(min = 18, max = 100))]
    pub age: u8,
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateUserURL {
    pub uuid: String,
}


#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub age: u8,
    pub email: String,
}

impl User {
    pub fn new(uuid: String, name: String, age: u8, email: String) -> Self {
        Self {
            uuid,
            name,
            age,
            email,
        }
    }
}