use mongodb::bson::oid::ObjectId;
use sqlx::{Encode, Postgres, Type};

#[async_trait::async_trait]
pub trait Repository<T, ID> {
    async fn find_all(&self) -> Result<Vec<T>, anyhow::Error>;
    async fn find_one(&self, id: &ID) -> Result<Option<T>, anyhow::Error>;
    async fn insert_one(&self, entity: &T) -> Result<ID, anyhow::Error>;
    async fn update_one(&self, id: &ID, entity: &T) -> Result<Option<T>, anyhow::Error>;
    async fn delete_one(&self, id: &ID) -> Result<u64, anyhow::Error>;
}

#[async_trait::async_trait]
pub trait MongoRepository<E, ID> {
    async fn find_all(&self) -> Result<Vec<E>, anyhow::Error>;
    async fn find_one(&self, id: &ID) -> Result<Option<E>, anyhow::Error>;
    async fn insert_one(&self, doc: &E) -> Result<ObjectId, anyhow::Error>;
    async fn update_one(&self, id: &ID, doc: &E) -> Result<Option<E>, anyhow::Error>;
    async fn delete_one(&self, id: &ID) -> Result<u64, anyhow::Error>;
}


pub trait Entity {
    type Id;

    fn columns() -> &'static [&'static str];

    /// Return values generically, no dyn trait needed
    fn values<'e>(&'e self) -> Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>;
}