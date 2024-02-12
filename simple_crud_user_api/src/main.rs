use actix_web::{get, post, put, App, HttpResponse, HttpServer, Responder, delete};
use actix_web::web::{Data, Json, Path};
use validator::Validate;

use crate::models::{CreateUserRequest, UpdateUserURL};
use crate::db::Database;
use crate::error::UserError;
use crate::models::user::User;
mod models;
mod db;
mod error;



#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn get_users(db: Data<Database>) -> Result<Json<Vec<User>>, UserError> {
    let users = db.get_all_users().await;
    match users {
        Some(users) => Ok(Json(users)),
        None => Err(UserError::UserRetrievalFailed)
    }
}

#[post("/create_user")]
async fn create_user(body: Json<CreateUserRequest>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let user_name = body.name.clone();
            let user_age = body.age;
            let user_email = body.email.clone();

            if db.check_email_exists(&user_email).await {
                return Err(UserError::UserAlreadyExists);
            }

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_user = db.create_user(User::new(
                String::from(new_uuid),
                user_name,
                user_age,
                user_email
            )).await;

            match new_user {
                Some(user) => Ok(Json(user)),
                None => Err(UserError::UserCreationFailed)
            }


        },
        Err(_) => Err(UserError::UserCreationFailed)
    }
}

#[put("/update_user/{uuid}")]
async fn update_user(update_user_url: Path<UpdateUserURL>, body: Json<CreateUserRequest>, db: Data<Database>) -> Result<Json<User>, UserError> {
let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let user_name = body.name.clone();
            let user_age = body.age;
            let user_email = body.email.clone();

            let user = db.get_user_by_uuid(&update_user_url.uuid).await;

            match user {
                Some(user) => {
                    let updated_user = db.update_user(User::new(
                        user.uuid,
                        user_name,
                        user_age,
                        user_email
                    )).await;

                    match updated_user {
                        Some(user) => Ok(Json(user)),
                        None => Err(UserError::UserUpdateFailed)
                    }
                },
                None => Err(UserError::NoUsersFound)
            }
        },
        Err(_) => Err(UserError::UserUpdateFailed)
    }
}

#[delete("/delete_user/{uuid}")]
async fn delete_user(delete_user_url: Path<UpdateUserURL>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let user = db.get_user_by_uuid(&delete_user_url.uuid).await;

    match user {
        Some(_user) => {
            let deleted_user = db.delete_user(&delete_user_url.uuid).await;

            match deleted_user {
                Some(user) => Ok(Json(user)),
                None => Err(UserError::UserDeletionFailed)
            }
        },
        None => Err(UserError::NoUsersFound)
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Failed to initialize database");

    let db_data = Data::new(db);


    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(index)
        .service(get_users)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
