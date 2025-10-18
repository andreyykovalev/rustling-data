use crate::api::RepositoryError;
use sqlx::{Encode, Executor, FromRow, Postgres, Row, Type, postgres::PgRow};

pub struct PostgresDriver;

impl PostgresDriver {
    pub async fn find_all<'e, T, E>(
        executor: E,
        table: &str
    ) -> Result<Vec<T>, RepositoryError<sqlx::Error>>
    where
        E: Executor<'e, Database = Postgres>,
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {}", table);
        sqlx::query_as::<_, T>(&query)
            .fetch_all(executor)
            .await
            .map_err(RepositoryError::ConnectionError)
    }

    pub async fn insert<'e, E>(
        executor: E,
        table: &str,
        columns: &[&str],
        values: Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>,
    ) -> Result<i32, RepositoryError<sqlx::Error>>
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

        q.fetch_one(executor)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(db_err) => {
                    RepositoryError::ConstraintViolation(db_err.message().to_string())
                }
                other => RepositoryError::ConnectionError(other),
            })
            .and_then(|row| {
                row.try_get::<i32, _>("id")
                    .map_err(|e| RepositoryError::Other(e.to_string()))
            })
    }

    pub async fn find_one<'e, T, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
    ) -> Result<Option<T>, RepositoryError<sqlx::Error>>
    where
        E: Executor<'e, Database = Postgres>,
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {} WHERE {} = $1", table, id_column);
        sqlx::query_as::<_, T>(&query)
            .bind(id_value)
            .fetch_optional(executor)
            .await
            .map_err(RepositoryError::ConnectionError)
    }

    pub async fn update<'e, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
        columns: &[&str],
        values: Vec<&'e (impl Encode<'e, Postgres> + Type<Postgres>)>,
    ) -> Result<u64, RepositoryError<sqlx::Error>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let set_clause: Vec<String> = columns
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{} = ${}", c, i + 1))
            .collect();
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

        q.execute(executor)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(db_err) => {
                    RepositoryError::ConstraintViolation(db_err.message().to_string())
                }
                other => RepositoryError::ConnectionError(other),
            })
            .map(|res| res.rows_affected())
    }

    pub async fn delete<'e, E>(
        executor: E,
        table: &str,
        id_column: &str,
        id_value: i32,
    ) -> Result<u64, RepositoryError<sqlx::Error>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let query = format!("DELETE FROM {} WHERE {} = $1", table, id_column);
        let result = sqlx::query(&query)
            .bind(id_value)
            .execute(executor)
            .await
            .map_err(RepositoryError::ConnectionError)?;
        Ok(result.rows_affected())
    }
}
