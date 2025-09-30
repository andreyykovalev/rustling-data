use std::fmt;
// example/src/main.rs
use anyhow;
use rustling_api::Repository;
use rustling_derive::Repository;
use sqlx::FromRow;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    username: String,
}

#[derive(Repository)]
#[entity(User)]
#[id(i32)]
pub struct UserRepository {
    pool: sqlx::PgPool,
}

impl UserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
        .await?;

    //
    // let repo = SqlRepository::<User>::new(&pool, "users");
    // let users = repo.find_all().await?; // <-- make find_all async
    // println!("Found users: {:?}", users);
    //
    // Ok(())
    // <UserRepository as rustling_api::HelloWorld>::hello();
    let repo = UserRepository::new(pool.clone());
    // let users = <UserRepository as Repository<User, i32>>::find_all(&repo)?;
    let users = repo.find_all().await?;
    println!("Fetched {:?} users", users);
    Ok(())
}
