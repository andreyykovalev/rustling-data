use rustling_data::PostgresDriver;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use testcontainers_modules::postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

#[derive(Debug, FromRow, PartialEq)]
struct User {
    id: i32,
    name: String,
    email: String,
}

/// Helper to start a Postgres container for a single test.
async fn start_postgres_container() -> (PgPool, ContainerAsync<postgres::Postgres>) {
    let container = postgres::Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres container");

    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get mapped port");

    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        host_port
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    // Create table for tests
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

    (pool, container)
}

/// Start an isolated transaction for a test
async fn start_test_transaction(pool: &PgPool) -> Transaction<'_, Postgres> {
    pool.begin().await.expect("Failed to start transaction")
}

#[tokio::test]
async fn test_insert() {
    let (pool, container) = start_postgres_container().await;

    let mut tx = start_test_transaction(&pool).await;

    let id = PostgresDriver::insert(
        tx.as_mut(),
        "users",
        &["name", "email"],
        vec![&"Alice", &"alice@example.com"],
    )
    .await
    .expect("Insert failed");

    assert!(id > 0);

    tx.rollback().await.unwrap();

    container.rm().await.expect("Failed to remove container");
}

#[tokio::test]
async fn test_find_all() {
    let (pool, container) = start_postgres_container().await;

    let mut tx = start_test_transaction(&pool).await;

    let _id = PostgresDriver::insert(
        tx.as_mut(),
        "users",
        &["name", "email"],
        vec![&"Bob", &"bob@example.com"],
    )
    .await
    .expect("Insert failed");

    let users: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .expect("Find all failed");

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Bob");

    tx.rollback().await.unwrap();
    container.rm().await.expect("Failed to remove container");
}

#[tokio::test]
async fn test_find_one() {
    let (pool, container) = start_postgres_container().await;

    let mut tx = start_test_transaction(&pool).await;

    let id = PostgresDriver::insert(
        tx.as_mut(),
        "users",
        &["name", "email"],
        vec![&"Charlie", &"charlie@example.com"],
    )
    .await
    .expect("Insert failed");

    let user: Option<User> = PostgresDriver::find_one(tx.as_mut(), "users", "id", id)
        .await
        .expect("Find one failed");

    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.name, "Charlie");
    assert_eq!(user.email, "charlie@example.com");

    tx.rollback().await.unwrap();
    container.rm().await.expect("Failed to remove container");
}

#[tokio::test]
async fn test_update() {
    let (pool, container) = start_postgres_container().await;

    let mut tx = start_test_transaction(&pool).await;

    let id = PostgresDriver::insert(
        tx.as_mut(),
        "users",
        &["name", "email"],
        vec![&"Dave", &"dave@example.com"],
    )
    .await
    .expect("Insert failed");

    let updated_rows = PostgresDriver::update(
        tx.as_mut(),
        "users",
        "id",
        id,
        &["name", "email"],
        vec![&"David", &"david@example.com"],
    )
    .await
    .expect("Update failed");

    assert_eq!(updated_rows, 1);

    let updated_user: User = PostgresDriver::find_one(tx.as_mut(), "users", "id", id)
        .await
        .expect("Find one failed")
        .expect("User not found");

    assert_eq!(updated_user.name, "David");
    assert_eq!(updated_user.email, "david@example.com");

    tx.rollback().await.unwrap();
    container.rm().await.expect("Failed to remove container");
}

#[tokio::test]
async fn test_delete() {
    let (pool, container) = start_postgres_container().await;

    let mut tx = start_test_transaction(&pool).await;

    let id = PostgresDriver::insert(
        tx.as_mut(),
        "users",
        &["name", "email"],
        vec![&"Eve", &"eve@example.com"],
    )
    .await
    .expect("Insert failed");

    let deleted_rows = PostgresDriver::delete(tx.as_mut(), "users", "id", id)
        .await
        .expect("Delete failed");

    assert_eq!(deleted_rows, 1);

    let remaining: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .expect("Find all failed");

    assert!(remaining.is_empty());

    tx.rollback().await.unwrap();
    container.rm().await.expect("Failed to remove container");
}
