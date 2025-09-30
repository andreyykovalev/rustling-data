// rustling-core/src/lib.rs
use sqlx::{FromRow, Pool, Postgres, query_as};
use std::marker::PhantomData;

pub struct SqlRepository {
    pool: sqlx::PgPool,
}

impl SqlRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all<T>(&self, table: &str) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {}", table);
        Ok(sqlx::query_as::<_, T>(&query).fetch_all(&self.pool).await?)
    }

    pub fn hello() {
        println!("Hello, Wcscsorld!");
    }
}
