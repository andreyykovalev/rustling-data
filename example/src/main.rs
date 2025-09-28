// example/src/main.rs
use rustling_core::SqlRepository;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, FromRow};
use rustling_derive::Repository;

#[derive(Debug, Deserialize, FromRow, Repository)]
#[repository(entity = User, id = i32)]
struct User {
    id: i32,
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
        .await?;

    let repo = SqlRepository::<User>::new(&pool, "users");
    let users = repo.find_all().await?; // <-- make find_all async
    println!("Found users: {:?}", users);

    Ok(())
}