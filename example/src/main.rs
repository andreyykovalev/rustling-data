use anyhow;

use rustling_data::Client;
use rustling_data::ClientOptions;
use rustling_data::api::Repository;
use rustling_data::bson::oid::ObjectId;
use rustling_derive::{MongoRepository, Repository};

use rustling_data::PgPool;
use rustling_data::PgPoolOptions;
use serde::{Deserialize, Serialize};

pub use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
}

#[derive(Repository)]
#[entity(User)]
#[id(i32)]
pub struct UserRepository {
    pool: PgPool,
}

#[derive(MongoRepository)]
#[entity(Test)]
#[id(bson::oid::ObjectId)]
#[collection("tests")]
pub struct TestRepository {
    client: Client,
    db_name: String,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
        .await?;

    let repo = UserRepository::new(pool.clone());
    // let users = <UserRepository as Repository<User, i32>>::find_all(&repo)?;
    let users = repo.find_all().await?;
    println!("Fetched {:?} users", users);

    let client_options = ClientOptions::parse("mongodb://admin:secret@localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // Initialize repository
    let test_repo = TestRepository {
        client: client.clone(),
        db_name: "my_database".to_string(),
    };

    let existing = test_repo.find_all().await?;
    if existing.is_empty() {
        let new_test = Test {
            id: ObjectId::new(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        test_repo
            .client
            .database(&test_repo.db_name)
            .collection::<Test>("tests")
            .insert_one(new_test)
            .await?;
    }

    let tests = test_repo.find_all().await?;
    println!("All Test documents from MongoDB: {:?}", tests);
    Ok(())
}
