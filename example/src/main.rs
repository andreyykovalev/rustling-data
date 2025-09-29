// example/src/main.rs
use rustling_api::HelloWorld;
use anyhow;
use rustling_derive::HelloWorld;
use rustling_core;

#[derive(HelloWorld)]
struct UserRepository;

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
    UserRepository::hello();
    Ok(())
}