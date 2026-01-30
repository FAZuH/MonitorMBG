# Backend Development Guide

This guide covers development workflows, testing, and best practices for the MonitorMBG backend.

## Quick Start with dev.sh

We provide a convenient development script at `/dev.sh` for common tasks:

```bash
# Run tests
./dev.sh test

# Format code
./dev.sh format

# Run linter
./dev.sh lint

# Build for production
./dev.sh build

# Run all pre-commit checks
./dev.sh precommit
```

See `./dev.sh help` for all available commands and options.

## Prerequisites

- Rust 1.75+ (with cargo)
- PostgreSQL 15+
- Docker & Docker Compose (for local database)

## Getting Started

### 1. Start the Database

If you don't have PostgreSQL running locally, start it with Docker:

```bash
# Using dev.sh (recommended)
./dev.sh test  # Will auto-start DB if not running

# Or manually:
docker compose up --build -d db
```

This will start a PostgreSQL container with the configuration defined in `docker-compose.yml`.

### 2. Environment Setup

Create a `.env` file in the `/backend` directory:

```bash
cd backend
cp .env.example .env  # If exists, or create manually
```

Required environment variables:
- `DATABASE_URL`: PostgreSQL connection string (e.g., `postgres://postgres:password@localhost:5432/monitormbg`)
- `JWT_SECRET`: Secret key for JWT token generation
- `PORT`: Server port (default: 3000)
- `HOST`: Server host (default: 0.0.0.0)

#### Storage Configuration

The application supports multiple storage backends for file uploads:

**Local Storage (Default):**
```bash
STORAGE_TYPE=local
STORAGE_LOCAL_PATH=./uploads          # Directory for local file storage
STORAGE_BASE_URL=http://localhost:3000/uploads  # Base URL for file access
```

**Amazon S3 or S3-Compatible Storage:**
```bash
STORAGE_TYPE=s3
STORAGE_S3_BUCKET=my-bucket
STORAGE_S3_REGION=us-east-1
STORAGE_S3_ACCESS_KEY=your-access-key
STORAGE_S3_SECRET_KEY=your-secret-key
STORAGE_S3_ENDPOINT=https://s3.amazonaws.com  # Optional: for MinIO, etc.
STORAGE_BASE_URL=https://my-bucket.s3.amazonaws.com
```

**Configuration Options:**

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `STORAGE_TYPE` | Storage backend type: `local` or `s3` | No | `local` |
| `STORAGE_LOCAL_PATH` | Local directory path | No (if local) | `./uploads` |
| `STORAGE_S3_BUCKET` | S3 bucket name | Yes (if s3) | - |
| `STORAGE_S3_REGION` | AWS region | No | `us-east-1` |
| `STORAGE_S3_ACCESS_KEY` | AWS access key ID | Yes (if s3) | - |
| `STORAGE_S3_SECRET_KEY` | AWS secret access key | Yes (if s3) | - |
| `STORAGE_S3_ENDPOINT` | Custom S3 endpoint | No | - |
| `STORAGE_BASE_URL` | Base URL for file URLs | No | Auto-generated |

### 3. Run the Server

```bash
cd backend
cargo run
```

The server will start at `http://localhost:3000` (or your configured PORT).

## Testing

### Run All Tests

Using dev.sh (recommended):
```bash
./dev.sh test
```

Or manually:
```bash
cd backend
cargo test --all-features
```

This runs:
- Unit tests (inline in source files)
- Integration tests (in `/tests` directory)
- Database tests (creates isolated test databases)

### Database Tests

Integration tests automatically:
1. Create a fresh test database (isolated per test)
2. Run migrations
3. Execute tests
4. Clean up the database

If database connection fails during tests, ensure PostgreSQL is running:

```bash
docker compose up --build -d db
```

### Test Structure

```
backend/
├── src/
│   └── auth/
│       └── utils.rs          # Unit tests inline (#[cfg(test)])
├── tests/
│   ├── common/
│   │   └── mod.rs            # Test utilities (setup_db, teardown_db)
│   ├── auth_integration.rs   # Integration tests
│   ├── auth_security_test.rs # Security tests
│   └── db_table_test.rs      # Database CRUD tests
```

## Code Quality

### Formatting

Using dev.sh (recommended):
```bash
./dev.sh format
```

Or manually:
```bash
cd backend
cargo +nightly fmt --all
```

### Linting

Using dev.sh (recommended):
```bash
./dev.sh lint
```

Or manually:
```bash
cd backend
cargo clippy --all-targets --all-features --fix --allow-dirty
```

### Pre-commit Checklist

Using dev.sh (recommended):
```bash
./dev.sh precommit
```

Or manually:
```bash
# 1. Format code
cargo +nightly fmt --all

# 2. Run linter
cargo clippy --all-targets --all-features --fix --allow-dirty

# 3. Run tests
cargo test --all-features

# 4. Check compilation
cargo check --all-features
```

## Architecture Overview

### Layered Architecture

```
HTTP Request
    ↓
Handler (routes/)      - Parse input, validate, call service
    ↓
Service (service/)     - Business logic, orchestration
    ↓
Database (database/)   - Data access, SQL queries
    ↓
PostgreSQL
```

### Storage Service Architecture

The storage service provides an abstraction layer for file uploads, supporting multiple backends:

```
UtilityService
    ↓
StorageService (Arc<StorageBackend>)
    ↓
┌─────────────────┬─────────────────┐
↓                 ↓                 ↓
LocalStorage    S3Storage        (Extensible)
(Local FS)      (AWS S3/MinIO)    (Future backends)
```

**Key Features:**
- **Pluggable Backends**: Switch between local filesystem and S3 without code changes
- **Unified API**: Same interface regardless of storage backend
- **Health Checks**: Storage status monitored in health check endpoint
- **Automatic Organization**: Files organized by date (YYYY/MM/DD)
- **UUID Generation**: Unique file identifiers for collision avoidance

### Key Components

1. **Handlers** (`src/routes/`)
   - Entry points for HTTP requests
   - Input validation and sanitization
   - **Rule**: No business logic or DB queries here

2. **Services** (`src/service/`)
   - Business logic implementation
   - Use `Arc<Database>` for data access
   - Convert between DB models and API DTOs

3. **Database Layer** (`src/database/`)
   - `model.rs`: Database table structs
   - `table.rs`: CRUD operations via `Table` trait
   - `StatsQueries`: Complex statistical queries
   - **Rule**: All SQL queries live here

### Database Access Pattern

```rust
// Service layer uses Arc<Database>
pub struct KitchenService {
    db: Arc<Database>,
}

impl KitchenService {
    pub async fn get_kitchen(&self, id: Uuid) -> Result<KitchenDto, AppError> {
        // Access tables through Database
        let kitchen = self.db.kitchen_table.select(&id).await?;
        // ... business logic
    }
}
```

## Adding New Features

### 1. Database Changes

If you need new tables or queries:

1. Add model to `src/database/model.rs`
2. Add table implementation in `src/database/table.rs` using `impl_table!` macro
3. Add custom query methods to the table struct
4. Add migration file in `/migrations/`

### 2. Service Layer

1. Create or modify service in `src/service/`
2. Define DTOs for API responses
3. Implement business logic
4. Use `Arc<Database>` for data access

### 3. Handler Layer

1. Add routes in `src/routes/`
2. Parse and validate input
3. Call appropriate service methods
4. Return proper HTTP responses

### 4. Documentation

**All new code must have docstrings:**

```rust
/// Gets kitchen statistics computed from reviews.
///
/// # Arguments
/// * `kitchen_id` - The UUID of the kitchen
///
/// # Returns
/// * `Ok(KitchenStatsDto)` - Statistics including ratings and review counts
/// * `Err(AppError::NotFound)` - If kitchen doesn't exist
///
/// # Example
/// ```
/// let stats = service.get_kitchen_stats(id).await?;
/// ```
pub async fn get_kitchen_stats(&self, id: Uuid) -> Result<KitchenStatsDto, AppError> {
    // ...
}
```

## Debugging

### Enable Logging

Set `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

Levels: `error`, `warn`, `info`, `debug`, `trace`

### Database Inspection

Connect to the database:

```bash
psql postgres://postgres:password@localhost:5432/monitormbg
```

Common queries:

```sql
-- List all tables
\dt

-- Check migrations status
SELECT * FROM _sqlx_migrations;

-- View recent reviews
SELECT * FROM reviews ORDER BY created_at DESC LIMIT 10;
```

## Common Issues

### Database Connection Failed

```
Error: connection refused
```

**Solution**: Start PostgreSQL:
```bash
docker compose up --build -d db
```

### Migration Errors

If migrations fail, you may need to reset:

```bash
# Drop and recreate database
docker compose down -v
docker compose up --build -d db

# Run migrations
cd backend && cargo run
```

### Test Database Cleanup

If test databases accumulate:

```bash
# Connect to PostgreSQL and drop test databases
psql postgres://postgres:password@localhost:5432/postgres
DROP DATABASE monitor_mbg_test_<uuid>;
```

## API Documentation

See [api-schema.md](./api-schema.md) for complete API documentation including:
- Endpoint specifications
- Request/response schemas
- Authentication requirements
- Error codes
