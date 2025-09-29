use std::fmt;
// example/src/main.rs
use rustling_api::Repository;
use anyhow;
use rustling_derive::Repository;

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
}

#[derive(Debug, Repository)]
#[entity(User)]
#[id(i32)]
struct UserRepository;

impl fmt::Display for UserRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UserRepository")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
    //     .await?;
    //
    // let repo = SqlRepository::<User>::new(&pool, "users");
    // let users = repo.find_all().await?; // <-- make find_all async
    // println!("Found users: {:?}", users);
    //
    // Ok(())
    // <UserRepository as rustling_api::HelloWorld>::hello();
    let repo = UserRepository;
    // let users = <UserRepository as Repository<User, i32>>::find_all(&repo)?;
    repo.find_all();

    let repo = UserRepository;
    let users = repo.find_all().await?;
    println!("Fetched {} users", users.len());
    Ok(())
}