# rustling-derive ⚙️

**Procedural macros** for generating repository implementations
using the [`rustling-data`](https://crates.io/crates/rustling-data) runtime.

---

## ✨ Macros Overview

| Macro | Description |
|--------|--------------|
| `#[derive(Repository)]` | Generates CRUD repository for PostgreSQL |
| `#[derive(MongoRepository)]` | Generates CRUD repository for MongoDB |
| `#[derive(Entity)]` | Generates metadata accessors for entity structs |

---

## 📦 Installation

```toml
[dependencies]
rustling-data = { version = "0.1", features = ["mongo"] }
rustling-derive = "0.1"