pub mod api;
pub mod drivers;

#[cfg(feature = "postgres")]
pub use drivers::postgres::PostgresDriver;
#[cfg(feature = "postgres")]
pub use sqlx::FromRow;
#[cfg(feature = "postgres")]
pub use sqlx::PgPool;
#[cfg(feature = "postgres")]
pub use sqlx::postgres::PgPoolOptions;

#[cfg(feature = "mongo")]
pub use drivers::mongo::MongoDriver;
#[cfg(feature = "mongo")]
pub use mongodb;
#[cfg(feature = "mongo")]
pub use mongodb::options::ClientOptions;
#[cfg(feature = "mongo")]
pub use mongodb::{Client, bson};
