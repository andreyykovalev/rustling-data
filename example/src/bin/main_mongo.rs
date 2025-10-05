use anyhow::Result;
use rustling_data::Client;
use rustling_data::ClientOptions;
use rustling_data::api::MongoRepository;
use rustling_derive::MongoRepository;
use rustling_data::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
}

#[derive(MongoRepository)]
#[entity(User)]
#[id(bson::oid::ObjectId)]
#[collection("users")]
pub struct UserRepository {
    client: Client,
    db_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client_options = ClientOptions::parse("mongodb://admin:secret@localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let repo = UserRepository {
        client,
        db_name: "my_database".to_string(),
    };

    let existing = repo.find_all().await?;
    if existing.is_empty() {
        let new_user = User {
            id: ObjectId::new(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        repo.client
            .database(&repo.db_name)
            .collection::<User>("users")
            .insert_one(new_user)
            .await?;
    }

    let users = repo.find_all().await?;
    println!("All User documents from MongoDB: {:?}", users);
    Ok(())
}
