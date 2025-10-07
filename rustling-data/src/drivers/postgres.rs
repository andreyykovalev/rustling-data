use sqlx::postgres::PgRow;
use sqlx::{Encode, Executor, FromRow, Postgres, Row, Type};

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

    pub async fn insert<'e, E>(
        executor: E,
        table: &str,
        columns: &[&str],
        values: Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>,
    ) -> Result<i32, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();
        let query_string = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );

        let box_str = query_string.into_boxed_str();
        let static_str: &'static str = Box::leak(box_str);
        let mut q = sqlx::query(static_str);
        for v in values {
            q = q.bind(v);
        }

        let row = q.fetch_one(executor).await?;
        let id: i32 = row.try_get("id")?;
        Ok(id)
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

    pub async fn update<'e, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
        columns: &[&str],
        values: Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>,
    ) -> Result<u64, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let set_clause: Vec<String> = columns.iter().enumerate().map(|(i, c)| format!("{} = ${}", c, i+1)).collect();
        let id_placeholder = columns.len() + 1;

        let query_string = format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            table,
            set_clause.join(", "),
            id_column,
            id_placeholder
        );

        let box_str = query_string.into_boxed_str();
        let static_str: &'static str = Box::leak(box_str);
        let mut q = sqlx::query(static_str);
        for v in values {
            q = q.bind(v);
        }

        q = q.bind(id_value);

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
