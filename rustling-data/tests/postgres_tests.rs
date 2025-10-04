use testcontainers_modules::{
    postgres,
    testcontainers::runners::AsyncRunner, // ✅ Use AsyncRunner for async mode
};
use tokio_postgres::NoTls;

#[tokio::test]
async fn test_postgres_async() {
    // 1. Start PostgreSQL container asynchronously
    //    `.start().await` returns a `ContainerAsync<Postgres>`
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
        "host=127.0.0.1 port={} user=postgres password=postgres dbname=postgres",
        host_port
    );

    // 4. Connect asynchronously using tokio-postgres
    let (client, connection) =
        tokio_postgres::connect(&connection_string, NoTls)
            .await
            .expect("Failed to connect to postgres testcontainer");

    // Spawn the connection task so it can process messages
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // 5. Run an async query
    let row = client
        .query_one("SELECT 1 + 1", &[])
        .await
        .expect("Query failed");

    // 6. Verify result
    let value: i32 = row.get(0);
    assert_eq!(value, 2, "SELECT 1 + 1 should return 2");

    // Container auto-stops when dropped ✅
}