use sqlx::postgres::PgRow;
use sqlx::{Executor, FromRow, Postgres};

pub struct PostgresDriver;
impl PostgresDriver {
    pub async fn find_all<'e, T, E>(executor: E, table: &str) -> Result<Vec<T>, sqlx::Error>
    where
        // E must be an Executor for Postgres, associated with the lifetime 'e.
        E: Executor<'e, Database = Postgres>,
        T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {}", table);
        // Execute against the generic executor
        Ok(sqlx::query_as::<_, T>(&query).fetch_all(executor).await?)
    }

    pub async fn insert<'e, T, E>(
        executor: E,
        table: &str,
        columns: &[&str],
        values: &'e [T],
    ) -> Result<u64, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
        T: sqlx::Encode<'e, Postgres> + sqlx::Type<Postgres> + Send + Sync + 'e,
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

        let box_str = query_string.into_boxed_str();
        let static_str: &'static str = Box::leak(box_str);
        let mut q = sqlx::query(static_str);

        for v in values {
            q = q.bind(v);
        }

        // Execute against the generic executor
        let result = q.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn find_one<'e, T, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
    ) -> Result<Option<T>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {} WHERE {} = $1", table, id_column);
        sqlx::query_as::<_, T>(&query)
            .bind(id_value)
            .fetch_optional(executor)
            .await
    }

    pub async fn update<'e, T, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
        updates: &'e [(&str, T)],
    ) -> Result<u64, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
        T: sqlx::Encode<'e, Postgres> + sqlx::Type<Postgres> + Send + Sync + 'e,
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

        // Execute against the generic executor
        let result = q.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete<'e, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
    ) -> Result<u64, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let query = format!("DELETE FROM {} WHERE {} = $1", table, id_column);
        let result = sqlx::query(&query)
            .bind(id_value)
            // Execute against the generic executor
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }
}
