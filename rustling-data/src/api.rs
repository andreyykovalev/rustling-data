use sqlx::{Encode, Postgres, Type};

#[async_trait::async_trait]
pub trait CrudRepository<T, ID> {
    async fn find_all(&self) -> Result<Vec<T>, anyhow::Error>;
    async fn find_one(&self, id: &ID) -> Result<Option<T>, anyhow::Error>;
    async fn insert_one(&self, entity: &T) -> Result<ID, anyhow::Error>;
    async fn update_one(&self, id: &ID, entity: &T) -> Result<Option<T>, anyhow::Error>;
    async fn delete_one(&self, id: &ID) -> Result<u64, anyhow::Error>;
}

pub trait PostgresEntity {
    type Id;
    fn columns() -> &'static [&'static str];
    fn values<'e>(&'e self) -> Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>;
}