use mongodb::bson::oid::ObjectId;

#[async_trait::async_trait]
pub trait Repository<T, ID> {
    async fn find_all(&self) -> Result<Vec<T>, anyhow::Error>;
}

#[async_trait::async_trait]
pub trait MongoRepository<E, ID> {
    async fn find_all(&self) -> Result<Vec<E>, anyhow::Error>;
    async fn find_one(&self, id: &ID) -> Result<Option<E>, anyhow::Error>;
    async fn insert_one(&self, doc: &E) -> Result<ObjectId, anyhow::Error>;
    async fn update_one(&self, id: &ID, doc: &E) -> Result<Option<E>, anyhow::Error>;
    async fn delete_one(&self, id: &ID) -> Result<u64, anyhow::Error>;
}