use rustling_data::PostgresDriver;
// tests/postgres_driver_crud.rs
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgPoolOptions;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres;
// Import the specific ContainerAsync type
use testcontainers_modules::testcontainers::ContainerAsync;

// The use of tokio_postgres::NoTls is not strictly required here if only sqlx::PgPool is used,
// as sqlx handles the connection. We can remove it for clarity.
// use tokio_postgres::NoTls;

#[derive(Debug, FromRow, PartialEq)]
struct User {
    id: i32,
    name: String,
    email: String,
}

/// Helper function to start a test Postgres container and return a connected pool
///
/// ⚠️ IMPORTANT: Due to the overhead of starting a new Docker container and PgPool
/// for every test, these tests MUST be run serially to avoid PoolTimedOut errors
/// caused by resource exhaustion and concurrency contention.
///
/// Run tests using: cargo test -- --test-threads=1
pub async fn setup_db() -> (PgPool, ContainerAsync<postgres::Postgres>) {
    // 1. Start PostgreSQL container asynchronously
    let container = postgres::Postgres::default()
        .start()
        .await
        .expect("Failed to start PostgreSQL test container");

    // 2. Get mapped port asynchronously
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get mapped port");

    // 3. Build connection string
    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        host_port
    );

    // 4. Connect asynchronously using PgPoolOptions
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    // 5. Create table for tests
    sqlx::query(
        r#"
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )
        "#,
    )
        .execute(&pool)
        .await
        .expect("Failed to create table");

    // 6. Return both the pool and the container handle
    (pool, container)
}

#[tokio::test]
async fn test_insert() {
    // Destructure, keeping _container in scope to keep the DB alive
    let (pool, _container) = setup_db().await;
    let driver = PostgresDriver::new(pool.clone());

    let inserted = driver
        .insert(
            "users",
            &["name", "email"],
            &[&"Alice", &"alice@example.com"],
        )
        .await
        .expect("Insert failed");

    assert_eq!(inserted, 1);
}

#[tokio::test]
async fn test_find_all() {
    // Destructure, keeping _container in scope to keep the DB alive
    let (pool, _container) = setup_db().await;
    let driver = PostgresDriver::new(pool.clone());

    driver
        .insert(
            "users",
            &["name", "email"],
            &[&"Bob", &"bob@example.com"],
        )
        .await
        .unwrap();

    let users: Vec<User> = driver.find_all("users").await.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Bob");
}

#[tokio::test]
async fn test_find_one() {
    // Destructure, keeping _container in scope to keep the DB alive
    let (pool, _container) = setup_db().await;
    // Changed `crate::PostgresDriver` to `PostgresDriver`
    let driver = PostgresDriver::new(pool.clone());

    driver
        .insert(
            "users",
            &["name", "email"],
            &[&"Charlie", &"charlie@example.com"],
        )
        .await
        .unwrap();

    let users: Vec<User> = driver.find_all("users").await.unwrap();
    let user: Option<User> = driver
        .find_one("users", "id", users[0].id)
        .await
        .unwrap();

    assert!(user.is_some());
    assert_eq!(user.unwrap().name, "Charlie");
}

#[tokio::test]
async fn test_update() {
    // Destructure, keeping _container in scope to keep the DB alive
    let (pool, _container) = setup_db().await;
    // Changed `crate::PostgresDriver` to `PostgresDriver`
    let driver = PostgresDriver::new(pool.clone());

    driver
        .insert(
            "users",
            &["name", "email"],
            &[&"Dave", &"dave@example.com"],
        )
        .await
        .unwrap();

    let users: Vec<User> = driver.find_all("users").await.unwrap();

    let updated_rows = driver
        .update(
            "users",
            "id",
            users[0].id,
            &[("name", &"David"), ("email", &"david@example.com")],
        )
        .await
        .unwrap();

    assert_eq!(updated_rows, 1);

    let updated_user: User = driver.find_one("users", "id", users[0].id).await.unwrap().unwrap();
    assert_eq!(updated_user.name, "David");
    assert_eq!(updated_user.email, "david@example.com");
}

#[tokio::test]
async fn test_delete() {
    // Destructure, keeping _container in scope to keep the DB alive
    let (pool, _container) = setup_db().await;
    // Changed `crate::PostgresDriver` to `PostgresDriver`
    let driver = PostgresDriver::new(pool.clone());

    driver
        .insert(
            "users",
            &["name", "email"],
            &[&"Eve", &"eve@example.com"],
        )
        .await
        .unwrap();

    let users: Vec<User> = driver.find_all("users").await.unwrap();
    let deleted_rows = driver
        .delete("users", "id", users[0].id)
        .await
        .unwrap();

    assert_eq!(deleted_rows, 1);

    let remaining: Vec<User> = driver.find_all("users").await.unwrap();
    assert!(remaining.is_empty());
}
