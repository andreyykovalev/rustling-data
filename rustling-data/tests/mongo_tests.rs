use futures_util::stream::TryStreamExt;
use mongodb::{
    Client, Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use testcontainers_modules::{
    mongo,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use tokio::time::sleep;
use rustling_data::MongoDriver;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

pub async fn setup_mongo() -> (Client, Database, ContainerAsync<mongo::Mongo>) {
    let container = mongo::Mongo::default().start().await.unwrap();
    let port = container.get_host_port_ipv4(27017).await.unwrap();
    let uri = format!("mongodb://localhost:{port}/testdb");

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

#[tokio::test]
async fn test_insert_one() {
    let (_client, _db, _container) = setup_mongo().await;

    let user = User {
        id: None,
        name: "Alice".into(),
        email: "alice@example.com".into(),
    };

    let mongo_repo = MongoDriver::new(_client.clone(), _db.name().to_string());

    let inserted_id = mongo_repo.insert_one("users", &user).await.unwrap();

    let found: User = mongo_repo
        .find_one("users", doc! { "_id": inserted_id })
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found.name, "Alice");
    assert_eq!(found.email, "alice@example.com");
}

#[tokio::test]
async fn test_find_all() {
    let (_client, _db, _container) = setup_mongo().await;
    let mongo_repo = MongoDriver::new(_client.clone(), _db.name().to_string());

    mongo_repo.insert_one(
        "users",
        &User {
            id: None,
            name: "Bob".into(),
            email: "bob@example.com".into(),
        },
    )
        .await
        .unwrap();

    let users: Vec<User> = mongo_repo.find_all("users").await.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Bob");
}

#[tokio::test]
async fn test_find_one() {
    let (_client, _db, _container) = setup_mongo().await;
    let mongo_repo = MongoDriver::new(_client.clone(), _db.name().to_string());

    mongo_repo
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

    let found: User = mongo_repo
        .find_one("users", doc! { "email": "charlie@example.com" })
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found.name, "Charlie");
}

#[tokio::test]
async fn test_update_one() {
    let (_client, _db, _container) = setup_mongo().await;
    let mongo_repo = MongoDriver::new(_client.clone(), _db.name().to_string());

    mongo_repo
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

    // update the document
    mongo_repo
        .update_one::<User>(
            "users",
            doc! { "email": "david@example.com" },
            doc! { "name": "Dave" },
        )
        .await
        .unwrap();

    let found: User = mongo_repo
        .find_one("users", doc! { "email": "david@example.com" })
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found.name, "Dave");
}

#[tokio::test]
async fn test_delete_one() {
    let (_client, _db, _container) = setup_mongo().await;
    let mongo_repo = MongoDriver::new(_client.clone(), _db.name().to_string());

    mongo_repo
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

    let deleted = mongo_repo
        .delete_one("users", doc! { "email": "eve@example.com" })
        .await
        .unwrap();
    assert_eq!(deleted, 1);

    let users: Vec<User> = mongo_repo.find_all("users").await.unwrap();
    assert!(users.is_empty());
}