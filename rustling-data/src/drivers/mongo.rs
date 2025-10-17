use crate::bson::to_document;
use futures::stream::TryStreamExt;
use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc, oid::ObjectId},
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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

    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: Send + Sync + Unpin + 'static,
    {
        self.db().collection::<T>(name)
    }

    pub async fn insert_one<T>(&self, collection: &str, doc: &T) -> Result<ObjectId, anyhow::Error>
    where
        T: Serialize + Send + Sync,
    {
        let coll = self.db().collection::<T>(collection);
        let result = coll.insert_one(doc).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    pub async fn find_all<T>(&self, collection: &str) -> Result<Vec<T>, anyhow::Error>
    where
        T: for<'de> Deserialize<'de> + Unpin + Send + Sync,
    {
        let coll = self.db().collection::<T>(collection);
        let mut cursor = coll.find(doc! {}).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    pub async fn find_one<T>(
        &self,
        collection: &str,
        filter: Document,
    ) -> Result<Option<T>, anyhow::Error>
    where
        T: for<'de> Deserialize<'de> + Unpin + Send + Sync,
    {
        let coll = self.db().collection::<T>(collection);
        let doc = coll.find_one(filter).await?;
        Ok(doc)
    }

    pub async fn update_one<T>(
        &self,
        collection: &str,
        filter: Document,
        doc: &T,
    ) -> Result<Option<T>, anyhow::Error>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        let coll = self.db().collection::<T>(collection);

        let mut update_doc = to_document(doc)?;
        update_doc.remove("_id"); // important
        let update_doc = doc! { "$set": update_doc };

        coll.find_one_and_update(filter.clone(), update_doc).await?;
        let result: Option<T> = coll.find_one(filter).await?;
        Ok(result)
    }

    pub async fn delete_one(
        &self,
        collection: &str,
        filter: Document,
    ) -> Result<u64, anyhow::Error> {
        let coll = self.db().collection::<Document>(collection);
        let result = coll.delete_one(filter).await?;
        Ok(result.deleted_count)
    }
}
