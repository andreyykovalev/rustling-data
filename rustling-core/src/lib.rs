// rustling-core/src/lib.rs
use sqlx::{Pool, Postgres, query_as, FromRow};
use std::marker::PhantomData;

pub struct SqlRepository<'a, T> {
    pub pool: &'a Pool<Postgres>,
    pub table_name: &'a str,
    _marker: PhantomData<T>,
}

impl<'a, T> SqlRepository<'a, T>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    pub fn new(pool: &'a Pool<Postgres>, table_name: &'a str) -> Self {
        Self {
            pool,
            table_name,
            _marker: PhantomData,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<T>, sqlx::Error> {
        let stmt = format!("SELECT * FROM {}", self.table_name);
        query_as::<_, T>(&stmt).fetch_all(self.pool).await
    }
}
