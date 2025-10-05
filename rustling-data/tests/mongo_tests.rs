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
    let (_client, db, _container) = setup_mongo().await;
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
    let (_client, db, _container) = setup_mongo().await;
    let collection = db.collection::<User>("users");

    collection
        .insert_one(User {
            id: None,
            name: "Bob".into(),
            email: "bob@example.com".into(),
        })
        .await
        .unwrap();

    let users: Vec<User> = collection
        .find(doc! {})
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Bob");
}

#[tokio::test]
async fn test_find_one() {
    let (_client, db, _container) = setup_mongo().await;
    let collection = db.collection::<User>("users");

    collection
        .insert_one(User {
            id: None,
            name: "Charlie".into(),
            email: "charlie@example.com".into(),
        })
        .await
        .unwrap();

    let found = collection
        .find_one(doc! { "email": "charlie@example.com" })
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Charlie");
}

#[tokio::test]
async fn test_update_one() {
    let (_client, db, _container) = setup_mongo().await;
    let collection = db.collection::<User>("users");

    collection
        .insert_one(User {
            id: None,
            name: "David".into(),
            email: "david@example.com".into(),
        })
        .await
        .unwrap();

    collection
        .update_one(
            doc! { "email": "david@example.com" },
            doc! { "$set": { "name": "Dave" } },
        )
        .await
        .unwrap();

    let found = collection
        .find_one(doc! { "email": "david@example.com" })
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found.name, "Dave");
}

#[tokio::test]
async fn test_delete_one() {
    let (_client, db, _container) = setup_mongo().await;
    let collection = db.collection::<User>("users");

    collection
        .insert_one(User {
            id: None,
            name: "Eve".into(),
            email: "eve@example.com".into(),
        })
        .await
        .unwrap();

    let delete_result = collection
        .delete_one(doc! { "email": "eve@example.com" })
        .await
        .unwrap();
    assert_eq!(delete_result.deleted_count, 1);

    let remaining: Vec<User> = collection
        .find(doc! {})
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert!(remaining.is_empty());
}
