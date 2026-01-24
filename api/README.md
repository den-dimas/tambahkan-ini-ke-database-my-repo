# Tambahkan Ini Ke Database (API)

A robust, high-performance REST API built with **Rust** and **Axum**.

## 🚀 Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/) (2024 edition)
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Database Wrapper:** [SQLx](https://github.com/launchbadge/sqlx) (PostgreSQL)
- **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
- **Authentication:** JWT with [Argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2) for password hashing
- **Object Storage:** Cloudflare R2 / S3-compatible storage
- **Caching:** [Moka](https://github.com/moka-rs/moka)
- **Rate Limiting:** `tower-governor`

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [PostgreSQL](https://www.postgresql.org/)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)

### Environment Setup

Create a `.env` file in the `api/` directory (refer to `.env` for required fields):

```env
DB_URL=postgresql://user:password@localhost:5432/dbname
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
PORT=8080
JWT_SECRET=your_secret_key
CORS_ORIGINS=http://localhost:4000
```

### Database Migrations

This project uses `sqlx-cli` for database migrations.

**Run migrations:**

```bash
sqlx migrate run
```

**Revert migrations:**

```bash
sqlx migrate revert
```

**Create a new migration:**

```bash
sqlx migrate add <migration_name>
```

### Development

Start the API server:

```bash
cargo run
```

### Testing

Run API tests:

```bash
cargo test
```

## 📂 Project Structure

- `src/routes/`: Route handlers and endpoint definitions.
- `src/services/`: Business logic and database interaction layer.
- `src/models/`: Data structures and SQLx entity definitions.
- `src/middleware/`: Custom Axum middleware (auth, rate limiting).
- `src/config/`: Application configuration and environment loading.
- `src/state.rs`: Shared application state.
- `src/utils/`: Generic utility functions.

## 🛡️ Security

- **Password Hashing:** Argon2id for secure password storage.
- **JWT:** Stateless authentication using signed JSON Web Tokens.
- **Rate Limiting:** Protects endpoints from abuse.
- **CORS:** Controlled access for the frontend.
