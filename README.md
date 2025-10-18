# Rustling 🦀

**Rustling** is a modular data access toolkit for Rust, providing
derive-based repository macros and database drivers for both **PostgreSQL** and **MongoDB**.
It provides a ready-to-use repository for your struct without a manual boilerplate.

This repository contains:

* 🧩 **rustling-data** — Core library (drivers, traits, repository logic)
* ✨ **rustling-derive** — Procedural macros (`#[derive(Repository)]`, `#[derive(MongoRepository)]`, etc.)
* 🧪 **example** — Sample project demonstrating usage

## 📦 Crates

| Crate                                  | Description                                    | Docs                                       |
| -------------------------------------- | ---------------------------------------------- | ------------------------------------------ |
| [`rustling-data`](./rustling-data)     | Core data layer, repository traits, DB drivers | [docs.rs](https://docs.rs/rustling-data)   |
| [`rustling-derive`](./rustling-derive) | Derive macros for Mongo/Postgres repositories  | [docs.rs](https://docs.rs/rustling-derive) |
| [`example`](./example)                 | Example project demonstrating usage            | -                                          |

## 🧰 Development

```bash
# Run all tests
cargo test

# Run tests for only Mongo
cargo test --test mongo_tests
```

## 🚀 Running Examples

This project includes several example binaries demonstrating different functionality.

### 🐘 Running PostgreSQL with Docker

You can start a PostgreSQL container with credentials used in examples of this project:

```bash
docker run --name rustling-postgres \
  -e POSTGRES_USER=rustling \
  -e POSTGRES_PASSWORD=secretpassword \
  -e POSTGRES_DB=rustlingdb \
  -p 5432:5432 \
  -d postgres
```

### Running an Example from the `/examples` Folder

```bash
cargo run --example main_postgres
```

* `-p example` specifies the package name.
* `--bin main_postgres` specifies the binary target within that package.

Workspace structure example:

```
rustling-data/
├─ Cargo.toml
├─ rustling-derive/
│  ├─ Cargo.toml
│  └─ src/
│     └─ lib.rs
├─ rustling-data/
│  ├─ Cargo.toml
│  └─ src/
│     └─ lib.rs
├─ example/
│  ├─ Cargo.toml
│  └─ src/
│     └─ bin/
│        ├─ main_postgres.rs
│        └─ main_mongo.rs
```

### Using `#[derive(Repository)]`

You can quickly create repositories for your data structs using the derive macros. Example:

```rust
use rustling_data::{PgPool, PgPoolOptions};
use rustling_data::api::CrudRepository;
use rustling_derive::{Entity, Repository};

#[derive(Debug, FromRow, Entity)]
#[derive(Clone)]
struct User {
    id: i32,
    username: String,
}

#[derive(Repository)]
#[entity(User)]
#[id(i32)]
pub struct UserRepository {
    pool: PgPool,
}

fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
        .await?;

    let repo = UserRepository { pool: pool.clone() };
    
    // Example: fetch all users
    let users = repo.find_all().await?;
    println!("All users: {:?}", users);
}
```

### Error Handling

```rust
#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustling:secretpassword@localhost:5432/rustlingdb")
        .await
        .expect("Failed to connect to database");

    let repo = UserRepository { pool: pool.clone() };

    match repo.find_all().await {
        Ok(users) => println!("All users: {:?}", users),
        Err(err) => match err {
            rustling_data::api::RepositoryError::NotFound => {
                eprintln!("No users found");
            },
            rustling_data::api::RepositoryError::ConnectionError(e) => {
                eprintln!("Database connection error: {:?}", e);
            },
            rustling_data::api::RepositoryError::ConstraintViolation(msg) => {
                eprintln!("Constraint violation: {}", msg);
            },
            rustling_data::api::RepositoryError::Other(msg) => {
                eprintln!("Other error: {}", msg);
            },
        },
    }
}
```

Handling **specific `RepositoryError` variants** when calling `find_all`, allowing different behaviors for not found entities, connection issues, constraint violations, or other errors.