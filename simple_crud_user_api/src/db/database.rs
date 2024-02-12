use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use crate::models::user::User;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("localhost:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root",
        })
            .await?;
        client.use_ns("surreal").use_db("users").await.expect("Failed to use database");
        Ok(Database {
            client,
            name_space: "surreal".to_string(),
            db_name: "users".to_string(),
        })
    }

    pub async fn get_all_users (&self) -> Option<Vec<User>> {
        let result = self.client.select("users").await;
        match result {
            Ok(users) => {
                let users: Vec<User> = users.into();
                Some(users)
            },
            Err(_) => None
        }
    }

    pub async fn create_user (&self, new_user: User) -> Option<User>{
        let created_user = self
            .client
            .create(("users", &new_user.uuid))
            .content(new_user)
            .await;

        created_user.unwrap_or_else(|_| None)
    }

    pub async fn update_user (&self, updated_user: User) -> Option<User>{
        let updated_user = self
            .client
            .update(("users", &updated_user.uuid))
            .content(updated_user)
            .await;

        updated_user.unwrap_or_else(|_| None)
    }

    pub async fn get_user_by_uuid (&self, uuid: &str) -> Option<User> {
        let user = self.client.select(("users", uuid)).await;
        user.unwrap_or_else(|_| None)
    }

    pub async fn delete_user (&self, uuid: &str) -> Option<User> {
        let user = self.client.delete(("users", uuid)).await;
        user.unwrap_or_else(|_| None)
    }

    pub async fn check_email_exists(&self, email: &str) -> bool {
        let users = self.get_all_users().await;
        match users {
            Some(users) => users.iter().any(|user| user.email == email),
            None => false,
        }
    }
}