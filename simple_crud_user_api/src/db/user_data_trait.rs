use crate::models::user::User;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;

#[async_trait]
pub trait UserDataTrait {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn create_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn update_user(db: &Data<Database>, updated_user: User) -> Option<User>;
    async fn get_user_by_uuid(db: &Data<Database>, uuid: &str) -> Option<User>;
    async fn delete_user(db: &Data<Database>, uuid: &str) -> Option<User>;
    async fn check_email_exists(db: &Data<Database>, email: &str) -> bool;
}

#[async_trait]
impl UserDataTrait for Database{
    async fn get_all_users (db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("users").await;
        match result {
            Ok(users) => {
                let users: Vec<User> = users.into();
                Some(users)
            },
            Err(_) => None
        }
    }

    async fn create_user (db: &Data<Database>, new_user: User) -> Option<User>{
        let created_user = db
            .client
            .create(("users", &new_user.uuid))
            .content(new_user)
            .await;

        created_user.unwrap_or_else(|_| None)
    }

    async fn update_user (db: &Data<Database>, updated_user: User) -> Option<User>{
        let updated_user = db
            .client
            .update(("users", &updated_user.uuid))
            .content(updated_user)
            .await;

        updated_user.unwrap_or_else(|_| None)
    }

    async fn get_user_by_uuid (db: &Data<Database>, uuid: &str) -> Option<User> {
        let user = db.client.select(("users", uuid)).await;
        user.unwrap_or_else(|_| None)
    }

    async fn delete_user (db: &Data<Database>, uuid: &str) -> Option<User> {
        let user = db.client.delete(("users", uuid)).await;
        user.unwrap_or_else(|_| None)
    }

    async fn check_email_exists(db: &Data<Database>, email: &str) -> bool {
        let users = Database::get_all_users(db).await;
        match users {
            Some(users) => users.iter().any(|user| user.email == email),
            None => false,
        }
    }
}