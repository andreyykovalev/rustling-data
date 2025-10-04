use once_cell::sync::Lazy;
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

/// Structure to hold the globally shared database resources.
struct SharedDb {
    pool: PgPool,
    // The container handle must be stored here to prevent the container from being dropped.
    _container_handle: ContainerAsync<postgres::Postgres>,
}

/// Global static variable to initialize the PostgreSQL container and pool only once.
static SHARED_DB: Lazy<SharedDb> = Lazy::new(|| {
    std::thread::spawn(|| {
        // Use a simple current-thread runtime builder for this dedicated setup task.
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio setup runtime for thread")
            .block_on(async {
                // 1. Start PostgreSQL container asynchronously
                let container = postgres::Postgres::default()
                    .start()
                    .await
                    .expect("Failed to start PostgreSQL test container");

                // 2. Get mapped port
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

                // 5. Create table for tests (done once globally)
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

                // 6. Return the shared resources
                SharedDb {
                    pool,
                    _container_handle: container,
                }
            })
    })
    .join()
    .expect("Thread panicked during DB setup")
});

/// Starts a new isolated transaction for a single test.
/// All operations within the test will be rolled back, ensuring isolation.
pub async fn start_test_transaction() -> Transaction<'static, Postgres> {
    // Ensure the static DB is initialized
    let pool = &SHARED_DB.pool;

    // Begin a transaction on a connection acquired from the pool
    pool.begin().await.expect("Failed to start transaction")
}

#[tokio::test]
async fn test_insert() {
    // 1. Start isolated transaction
    let mut tx = start_test_transaction().await;
    // 2. Use the driver as a stateless utility, passing the transaction as the Executor
    let inserted = PostgresDriver::insert(
        tx.as_mut(), // Pass the transaction executor
        "users",
        &["name", "email"],
        &[&"Alice", &"alice@example.com"],
    )
    .await
    .expect("Insert failed");

    assert_eq!(inserted, 1);

    // 3. Rollback to clean up changes
    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn test_find_all() {
    let mut tx = start_test_transaction().await;

    // Insert data within the transaction
    PostgresDriver::insert(
        tx.as_mut(), // Pass the transaction executor
        "users",
        &["name", "email"],
        &[&"Bob", &"bob@example.com"],
    )
    .await
    .unwrap();

    // Find all data within the transaction
    let users: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Bob");

    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn test_find_one() {
    let mut tx = start_test_transaction().await;

    PostgresDriver::insert(
        tx.as_mut(), // Pass the transaction executor
        "users",
        &["name", "email"],
        &[&"Charlie", &"charlie@example.com"],
    )
    .await
    .unwrap();

    let users: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .unwrap();
    let user: Option<User> = PostgresDriver::find_one(
        tx.as_mut(), // Pass the transaction executor
        "users",
        "id",
        users[0].id,
    )
    .await
    .unwrap();

    assert!(user.is_some());
    assert_eq!(user.unwrap().name, "Charlie");

    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn test_update() {
    let mut tx = start_test_transaction().await;

    PostgresDriver::insert(
        tx.as_mut(), // Pass the transaction executor
        "users",
        &["name", "email"],
        &[&"Dave", &"dave@example.com"],
    )
    .await
    .unwrap();

    let users: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .unwrap();

    let updated_rows = PostgresDriver::update(
        tx.as_mut(), // Pass the transaction executor
        "users",
        "id",
        users[0].id,
        &[("name", &"David"), ("email", &"david@example.com")],
    )
    .await
    .unwrap();

    assert_eq!(updated_rows, 1);

    // Use the driver's find_one method to confirm the update
    let updated_user: User = PostgresDriver::find_one(tx.as_mut(), "users", "id", users[0].id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated_user.name, "David");
    assert_eq!(updated_user.email, "david@example.com");

    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn test_delete() {
    let mut tx = start_test_transaction().await;

    PostgresDriver::insert(
        tx.as_mut(), // Pass the transaction executor
        "users",
        &["name", "email"],
        &[&"Eve", &"eve@example.com"],
    )
    .await
    .unwrap();

    let users: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .unwrap();
    let deleted_rows = PostgresDriver::delete(
        tx.as_mut(), // Pass the transaction executor
        "users",
        "id",
        users[0].id,
    )
    .await
    .unwrap();

    assert_eq!(deleted_rows, 1);

    let remaining: Vec<User> = PostgresDriver::find_all(tx.as_mut(), "users")
        .await
        .unwrap();
    assert!(remaining.is_empty());

    tx.rollback().await.unwrap();
}
