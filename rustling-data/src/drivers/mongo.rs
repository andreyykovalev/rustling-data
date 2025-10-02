use mongodb::{Client, Database};
use mongodb::bson::doc;
use futures::stream::TryStreamExt;

pub struct MongoDriver {
    client: Client,
    db_name: String,
}

impl MongoDriver {
    pub fn new(client: Client, db_name: impl Into<String>) -> Self {
        Self {
            client,
            db_name: db_name.into(),
        }
    }

    fn db(&self) -> Database {
        self.client.database(&self.db_name)
    }

    pub async fn find_all<T>(&self, collection: &str) -> Result<Vec<T>, anyhow::Error>
    where
        T: for<'de> serde::Deserialize<'de> + Unpin + Send + Sync,
    {
        let coll = self.db().collection::<T>(collection);
        let mut cursor = coll.find(doc! {}, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }
}