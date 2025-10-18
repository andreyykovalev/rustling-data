//! # rustling-data 🦀
//!
//! Core runtime layer for the **Rustling ORM** system.
//! Provides traits, error types, and database drivers used by the derive macros in
//! [`rustling-derive`](https://crates.io/crates/rustling-derive).
//!
//! ## Features
//!
//! - `mongo`: MongoDB driver
//! - `postgres`: PostgreSQL driver
//!
//! ## Example
//! ```rust,no_run
//! use rustling_data::api::CrudRepository;
//! use rustling_derive::MongoRepository;
//! ```
//!
//! See the crate README for complete examples.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-512x512.png")]

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
