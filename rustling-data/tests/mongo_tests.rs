use mongodb::{bson::{doc, oid::ObjectId}, Client};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use testcontainers_modules::{
    mongo,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};
use std::time::Duration;
use tokio::time::sleep;
use mongodb::Database;

use rustling_data::MongoDriver;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

struct SharedMongo {
    driver: MongoDriver,
    _container: ContainerAsync<mongo::Mongo>,
}

// Shared test container setup
static SHARED_MONGO: Lazy<SharedMongo> = Lazy::new(|| {
    std::thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                // 1️⃣ Start Mongo container
                let container = mongo::Mongo::default()
                    .start()
                    .await
                    .expect("Failed to start Mongo test container");

                // 2️⃣ Resolve host port
                let port = container
                    .get_host_port_ipv4(27017)
                    .await
                    .expect("Failed to get mapped port");

                // 3️⃣ Mongo URI (testcontainers uses admin/secret)
                let uri = format!("mongodb://admin:secret@127.0.0.1:{port}/");

                // 4️⃣ Connect client
                let client = Client::with_uri_str(&uri)
                    .await
                    .expect("Failed to connect to Mongo");

                // 5️⃣ Build driver
                let driver = MongoDriver::new(client, "test_db");

                SharedMongo {
                    driver,
                    _container: container,
                }
            })
    })
        .join()
        .unwrap()
});

pub async fn setup_mongo() -> (Client, Database, testcontainers_modules::testcontainers::ContainerAsync<mongo::Mongo>) {
    // Start the container
    let container = mongo::Mongo::default()
        .start()
        .await
        .expect("Failed to start Mongo container");

    // Get the mapped port (to connect from host)
    let port = container
        .get_host_port_ipv4(27017)
        .await
        .expect("Failed to get mapped port");

    let uri = format!("mongodb://localhost:{port}/testdb");

    // Wait for MongoDB to be ready
    let mut retries = 10;
    loop {
        match Client::with_uri_str(&uri).await {
            Ok(client) => {
                if client
                    .database("admin")
                    .run_command(doc! {"ping": 1})
                    .await
                    .is_ok()
                {
                    let db = client.database("testdb");
                    return (client, db, container);
                }
            }
            Err(_) => {}
        }

        if retries == 0 {
            panic!("MongoDB did not become ready in time");
        }

        retries -= 1;
        sleep(Duration::from_secs(1)).await;
    }
}

fn get_driver() -> &'static MongoDriver {
    &SHARED_MONGO.driver
}

async fn cleanup(driver: &MongoDriver) {
    let coll = driver.collection::<User>("users");
    coll.drop()
        .await
        .unwrap_or_else(|e| eprintln!("Drop failed: {e}"));
}

#[tokio::test]
async fn test_insert_one() {
    let (client, db, _container) = setup_mongo().await;
    let collection = db.collection::<User>("users");

    let user = User {
        id: None,
        name: "Alice".into(),
        email: "alice@example.com".into(),
    };

    collection.insert_one(user.clone()).await.unwrap();

    let found = collection
        .find_one(doc! { "email": "alice@example.com" })
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found.name, "Alice");
}

#[tokio::test]
async fn test_find_all() {
    let driver = get_driver();
    cleanup(driver).await;

    driver
        .insert_one(
            "users",
            &User {
                id: None,
                name: "Bob".into(),
                email: "bob@example.com".into(),
            },
        )
        .await
        .unwrap();

    let users: Vec<User> = driver.find_all("users").await.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Bob");
}

#[tokio::test]
async fn test_find_one() {
    let driver = get_driver();
    cleanup(driver).await;

    driver
        .insert_one(
            "users",
            &User {
                id: None,
                name: "Charlie".into(),
                email: "charlie@example.com".into(),
            },
        )
        .await
        .unwrap();

    let found = driver
        .find_one::<User>("users", doc! { "email": "charlie@example.com" })
        .await
        .unwrap();

    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Charlie");
}

#[tokio::test]
async fn test_update_one() {
    let driver = get_driver();
    cleanup(driver).await;

    driver
        .insert_one(
            "users",
            &User {
                id: None,
                name: "David".into(),
                email: "david@example.com".into(),
            },
        )
        .await
        .unwrap();

    let updated = driver
        .update_one::<User>(
            "users",
            doc! { "email": "david@example.com" },
            doc! { "name": "Dave" },
        )
        .await
        .unwrap();

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().name, "Dave");
}

#[tokio::test]
async fn test_delete_one() {
    let driver = get_driver();
    cleanup(driver).await;

    driver
        .insert_one(
            "users",
            &User {
                id: None,
                name: "Eve".into(),
                email: "eve@example.com".into(),
            },
        )
        .await
        .unwrap();

    let deleted = driver
        .delete_one("users", doc! { "email": "eve@example.com" })
        .await
        .unwrap();

    assert_eq!(deleted, 1);

    let remaining: Vec<User> = driver.find_all("users").await.unwrap();
    assert!(remaining.is_empty());
}
