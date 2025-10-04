use sqlx::postgres::PgRow;
use sqlx::{FromRow, Postgres};

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

    pub async fn insert<'a, T>(
        // 1. Re-introduce lifetime 'a
        &self,
        table: &str,
        columns: &[&str],
        values: &'a [T], // 2. Tie the input slice to lifetime 'a
    ) -> Result<u64, sqlx::Error>
    where
        // 3. FIX E0521: Use lifetime 'a for Encode.
        // This tells the compiler the borrowed data 'v' lives at least as long as 'a',
        // satisfying the borrow checker for the async operation.
        T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Send + Sync + 'a,
        // Note: Clone is removed as we now bind by reference.
    {
        if columns.len() != values.len() {
            return Err(sqlx::Error::Protocol(
                "Column count must match value count".to_string(),
            ));
        }

        let cols = columns.join(", ");
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();

        let query_string = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            cols,
            placeholders.join(", ")
        );

        // FIX E0597: Resolve query string lifetime by leaking the box. (Necessary for dynamic queries)
        let box_str = query_string.into_boxed_str();
        let static_str: &'static str = Box::leak(box_str);
        let mut q = sqlx::query(static_str);

        // 4. Bind by reference. The 'a lifetime makes this safe now.
        for v in values {
            q = q.bind(v);
        }

        let result = q.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn find_one<T>(
        &self,
        table: &str,
        id_column: &str,
        id_value: i32,
    ) -> Result<Option<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {} WHERE {} = $1", table, id_column);
        sqlx::query_as::<_, T>(&query)
            .bind(id_value)
            .fetch_optional(&self.pool)
            .await
    }
    pub async fn update<'a, T>(
        &self,
        table: &str,
        id_column: &str,
        id_value: i32,
        updates: &'a [(&str, T)], // Tie the input slice to lifetime 'a
    ) -> Result<u64, sqlx::Error>
    where
        // Use lifetime 'a for Encode. Remove Clone constraint.
        T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Send + Sync + 'a,
    {
        // 1. Build the SET clause and calculate the ID placeholder index
        let set_clause: Vec<String> = (1..=updates.len())
            .map(|i| format!("{} = ${}", updates[i - 1].0, i))
            .collect();

        let id_placeholder_index = updates.len() + 1;

        let query_string = format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            table,
            set_clause.join(", "),
            id_column,
            id_placeholder_index
        );

        // Resolve query string lifetime by leaking the box.
        let box_str = query_string.into_boxed_str();
        let static_str: &'static str = Box::leak(box_str);

        let mut q = sqlx::query(static_str);

        // Bind the update values first (by reference)
        for (_, value) in updates {
            q = q.bind(value);
        }

        // Bind the ID value last (corresponds to $id_placeholder_index)
        q = q.bind(id_value);

        let result = q.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    // 🔹 Delete
    pub async fn delete(
        &self,
        table: &str,
        id_column: &str,
        id_value: i32,
    ) -> Result<u64, sqlx::Error> {
        let query = format!("DELETE FROM {} WHERE {} = $1", table, id_column);
        let result = sqlx::query(&query)
            .bind(id_value)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
