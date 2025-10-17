#[cfg(feature = "postgres")]
use sqlx::{Encode, Postgres, Type};
#[cfg(feature = "mongo")]
pub type MongoError = mongodb::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError<DB> {
    #[error("entity not found")]
    NotFound,
    #[error("database connection error: {0}")]
    ConnectionError(DB),
    #[error("constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("unknown error: {0}")]
    Unknown(String),
}

#[async_trait::async_trait]
pub trait CrudRepository<T, ID, DB> {
    async fn find_all(&self) -> Result<Vec<T>, RepositoryError<DB>>;
    async fn find_one(&self, id: &ID) -> Result<Option<T>, RepositoryError<DB>>;
    async fn insert_one(&self, entity: &T) -> Result<ID, RepositoryError<DB>>;
    async fn update_one(&self, id: &ID, entity: &T) -> Result<Option<T>, RepositoryError<DB>>;
    async fn delete_one(&self, id: &ID) -> Result<u64, RepositoryError<DB>>;
}

#[cfg(feature = "postgres")]
pub trait PostgresEntity {
    type Id;
    fn columns() -> &'static [&'static str];
    fn values<'e>(&'e self) -> Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>;
}
