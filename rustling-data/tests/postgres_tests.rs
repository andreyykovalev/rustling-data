// tests/postgres_sync_test.rs

use testcontainers_modules::{
    postgres,
    testcontainers::{
        runners::SyncRunner, // Key difference: Use SyncRunner
    },
};
use sync_postgres::{Client, NoTls};

#[test]
fn test_postgres_synchronous() {
    // 1. Define and start the PostgreSQL container synchronously
    //    .start() blocks the thread until the container is ready.
    let container = postgres::Postgres::default()
        .start()
        .expect("Failed to start PostgreSQL test container");

    // 2. Get the dynamically mapped host port
    //    This method is synchronous and returns the Result<u16, Error>.
    let host_port: u16 = container
        .get_host_port_ipv4(5432)
        .expect("Failed to get mapped port");

    // 3. Construct the connection string
    let connection_string = &format!(
        "host=127.0.0.1 port={} user=postgres password=postgres dbname=postgres",
        host_port
    );

    // 4. Connect to the database using the synchronous 'postgres' client
    let mut client = Client::connect(connection_string, NoTls)
        .expect("Failed to connect to postgres testcontainer");

    // 5. Execute a synchronous test query
    let rows = client
        .query("SELECT 1 + 1", &[])
        .expect("Query failed");

    // 6. Assert the result
    //    The 'postgres' crate returns a row struct you can index and cast.
    let value: i32 = rows[0].get(0);
    assert_eq!(value, 2, "The query 'SELECT 1 + 1' should return 2");

    // 7. Cleanup: The container is automatically stopped and removed
    //    when the 'container' variable goes out of scope (implements Drop).
}