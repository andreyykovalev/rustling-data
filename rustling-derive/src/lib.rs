//! # rustling-derive ⚙️
//!
//! Procedural macros for automatic repository generation in the **Rustling ORM** ecosystem.
//!
//! This crate provides convenient `#[derive(...)]` macros that automatically
//! implement repository and entity patterns for MongoDB and PostgreSQL.
//!
//! ## ✨ Available Macros
//!
//! - `#[derive(Repository)]` — derive a PostgreSQL repository implementation
//! - `#[derive(MongoRepository)]` — derive a MongoDB repository implementation
//! - `#[derive(Entity)]` — derive helper methods for SQL entities (columns & values)
//!
//! ## 💡 Example
//! ```rust,no_run
//! use rustling_derive::{Entity, Repository};
//!
//! #[derive(Entity)]
//! struct User {
//!     id: i32,
//!     name: String,
//!     email: String,
//! }
//!
//! #[derive(Repository)]
//! #[entity(User)]
//! #[id(i32)]
//! #[table("users")]
//! struct UserRepository;
//! ```
//!
//! See the [crate README](https://crates.io/crates/rustling-derive) for setup instructions.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

mod common;

#[cfg(feature = "mongo")]
mod mongo_macro;
#[cfg(feature = "postgres")]
mod postgres_macro;

#[cfg(feature = "postgres")]
#[proc_macro_derive(Repository, attributes(entity, id, table))]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    postgres_macro::repository_derive(input)
}

#[cfg(feature = "mongo")]
#[proc_macro_derive(MongoRepository, attributes(entity, id, collection))]
pub fn mongo_repository_derive(input: TokenStream) -> TokenStream {
    mongo_macro::mongo_repository_derive(input)
}

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields: Vec<_> = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .iter()
                .filter(|f| f.ident.as_ref().unwrap() != "id")
                .collect(),
            _ => panic!("Entity derive only supports named fields"),
        },
        _ => panic!("Entity derive only supports structs"),
    };

    let column_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();
    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let gene = quote! {
        impl #name {
            pub fn columns() -> &'static [&'static str] {
                &[#(#column_names),*]
            }

            pub fn values<'e>(&'e self) -> Vec<&'e (impl sqlx::Encode<'e, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>)> {
                vec![#(&self.#field_idents),*]
            }
        }
    };

    gene.into()
}
