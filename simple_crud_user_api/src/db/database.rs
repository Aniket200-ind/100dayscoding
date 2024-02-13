use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

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
}