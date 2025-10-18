# rustling-data 🦀

**`rustling-data`** provides the **runtime layer** for Rustling ORM-style repositories —
including database drivers, generic repository traits, and shared error handling.

It is designed to be used together with [`rustling-derive`](https://crates.io/crates/rustling-derive),
which provides `#[derive(...)]` macros that automatically generate repository implementations
for MongoDB and PostgreSQL.

---

## ✨ Features

- 🧩 **Unified repository trait** — define CRUD logic once, backend-agnostic.
- 🗃️ **MongoDB driver** (optional via `--features mongo`)
- 🐘 **PostgreSQL driver** (optional via `--features postgres`)
- 🎯 **Custom RepositoryError** for consistent error handling.
- ⚙️ Fully async with `tokio` + `async-trait`.

---

## 🧱 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rustling-data = { version = "0.1", features = ["mongo"] }      # or ["postgres"]
rustling-derive = "0.1"
