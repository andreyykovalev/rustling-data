use anyhow::Result;
use rustling_data::{PgPool, PgPoolOptions};
use rustling_data::api::CrudRepository;
use rustling_derive::{Entity, Repository};
use sqlx::FromRow;

#[derive(Debug, FromRow, Entity)]
#[derive(Clone)]
struct User {
    id: i32,
    username: String,
}

#[derive(Repository)]
#[entity(User)]
#[id(i32)]
pub struct UserRepository {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
        .await?;

    let repo = UserRepository { pool: pool.clone() };

    // --- INSERT ONE ---
    let new_user = User { id: 0, username: "alice".into() }; // id may be ignored if auto-increment
    let inserted_id = repo.insert_one(&new_user).await?;
    println!("Inserted user with ID: {:?}", inserted_id);

    // --- FIND ALL ---
    let users = repo.find_all().await?;
    println!("All users: {:?}", users);

    // --- FIND ONE ---
    let user = repo.find_one(&inserted_id).await?;
    println!("Found user: {:?}", user);

    // --- UPDATE ONE ---
    if let Some(mut u) = user.clone() {
        u.username = "alice_updated".into();
        let updated = repo.update_one(&inserted_id, &u).await?;
        println!("Updated user: {:?}", updated);
    }

    // --- DELETE ONE ---
    let deleted_count = repo.delete_one(&inserted_id).await?;
    println!("Deleted {} user(s)", deleted_count);

    Ok(())
}
