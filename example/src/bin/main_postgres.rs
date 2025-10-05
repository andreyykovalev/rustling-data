use anyhow::Result;
use rustling_data::{PgPool, PgPoolOptions};
use rustling_data::api::Repository;
use rustling_derive::Repository;     
use sqlx::FromRow;

#[derive(Debug, FromRow)]
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

    let repo = UserRepository { pool };
    let users = repo.find_all().await?;
    println!("Fetched {:?} users", users);
    Ok(())
}
