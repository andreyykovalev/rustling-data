// rustling-core/src/lib.rs
use sqlx::{FromRow, Pool, Postgres, query_as};
use std::marker::PhantomData;

pub struct SqlRepository {}

impl SqlRepository {
    // pub async fn find_all(&self) -> Result<Vec<T>, sqlx::Error> {
    //
    // }

    pub fn hello() {
        println!("Hello, Wcscsorld!");
    }
}
