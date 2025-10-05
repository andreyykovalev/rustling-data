use anyhow::Result;
use rustling_data::Client;
use rustling_data::ClientOptions;
use rustling_derive::MongoRepository;
use rustling_data::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use rustling_data::api::MongoRepository;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    // Connect to MongoDB
    let client_options = ClientOptions::parse("mongodb://admin:secret@localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // Initialize repository
    let repo = UserRepository {
        client,
        db_name: "my_database".to_string(),
    };

    // Find all users
    let existing = repo.find_all().await?;
    println!("All users: {:?}", existing);

    // Insert a new user
    if existing.is_empty() {
        let new_user = User {
            id: ObjectId::new(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        let id = repo.insert_one(&new_user).await?;
        println!("Inserted user with id: {:?}", id);
    }

    // Fetch all users again
    let users = repo.find_all().await?;
    println!("Users after insert: {:?}", users);

    // Find a user by ID
    if let Some(first_user) = users.first() {
        let user = repo.find_one(&first_user.id).await?;
        println!("Found by ID: {:?}", user);
    }

    // Update a user
    if let Some(mut first_user) = users.first().cloned() {
        first_user.email = "alice@newdomain.com".to_string();
        let updated = repo.update_one(&first_user.id, &first_user).await?;
        println!("Updated user: {:?}", updated);
    }

    // Delete a user
    if let Some(first_user) = users.first() {
        let deleted_count = repo.delete_one(&first_user.id).await?;
        println!("Deleted {} document(s)", deleted_count);
    }

    Ok(())
}