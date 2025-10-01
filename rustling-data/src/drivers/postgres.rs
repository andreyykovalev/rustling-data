pub struct PostgresDriver {
    pool: sqlx::PgPool,
}

impl PostgresDriver {
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
        println!("Hello, World!");
    }
}
