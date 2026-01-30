# AI Agent Instructions for MonitorMBG

This document provides context, architectural guidelines, and operational instructions for AI agents working on the MonitorMBG codebase.

## Project Overview
MonitorMBG is a monitoring system for "Makan Bergizi Gratis" (Free Nutritious Meals) program. It consists of a Rust backend and a React/TypeScript frontend.

## 📂 Backend (`/backend`)

### Technology Stack
- **Language**: Rust
- **Web Framework**: Axum 0.8
- **Database**: PostgreSQL with `sqlx` 0.8
- **Authentication**: JWT (jsonwebtoken), Argon2 (password hashing)
- **Logging**: `tracing` and `log` crates

### Architecture
The backend follows a layered architecture to separate concerns:

1.  **Routes (`src/routes/`)**:
    - Entry points for HTTP requests.
    - Responsible for parsing input (JSON), validating simple constraints, and calling the Service layer.
    - **Rule**: Do NOT put business logic or database queries here.

2.  **Service Layer (`src/service/`)**:
    - Contains business logic.
    - Orchestrates database operations and external services.
    - **MUST** use `Arc<Database>` for database access (not direct PgPool).
    - Example: `AuthService` handles hashing passwords and generating tokens.

3.  **Database Layer (`src/database/`)**:
    - **Models (`src/database/model.rs`)**: Structs representing DB tables.
    - **Tables (`src/database/table.rs`)**: Data access objects using the `Table` trait.
    - **StatsQueries (`StatsQueries`)**: Complex statistical queries (national/regional stats, trends).
    - **Rule**: Use `sqlx` for type-safe queries. Extend specific Table structs (e.g., `UserTable`) for custom queries like `find_by_unique_code`.
    - **Rule**: The `pool` field in `Database` is private - services must use `Arc<Database>`.

### Security Standards
- **Passwords**: Must be 8-32 characters. Always hash using `AuthService`.
- **Input**: Sanitize strings (trim) in handlers. Validate lengths.
- **Timing Attacks**: Use constant-time comparisons or dummy verification logic in login flows.
- **JWT**: Short expiration (1 hour).

### Testing Strategy
- **Unit Tests**: Place inside the module (e.g., `src/auth/utils.rs`).
- **Integration Tests (`tests/`):
    - Use `common::setup_db()` to create a fresh, isolated database for each test.
    - Use `common::teardown_db()` to clean up.
    - Test full HTTP flows using `tower::ServiceExt::oneshot`.

### Documentation Requirements
**ALL** new code must include docstrings:
- **Functions/Methods**: Describe purpose, parameters, return values, and errors
- **Structs/Enums**: Describe what they represent and their fields
- **Traits**: Describe the contract and required behavior
- **Modules**: Describe the module's purpose at the top of the file
- **Public APIs**: Must have comprehensive documentation with examples where appropriate

Use Rust doc comment format (`///` or `//!`).

## 📂 Frontend (`/frontend`)

### Technology Stack
- **Framework**: React 18+
- **Language**: TypeScript
- **Build Tool**: Vite
- **UI Library**: shadcn/ui (Radix UI + Tailwind CSS)
- **Styling**: Tailwind CSS (`globals.css`)

### Directory Structure
- **`src/components/ui/`**: Reusable, atomic components (Buttons, Inputs, Cards). Do not modify these unless updating the design system.
- **`src/components/{feature}/`**: Feature-specific components (e.g., `auth/`, `dashboard/`).
- **`src/types/`**: Shared TypeScript interfaces.

### Development Guidelines
- **Components**: Prefer functional components with hooks.
- **Styling**: Use Tailwind utility classes. Avoid inline styles.
- **Responsiveness**: Ensure designs work on mobile (see `use-mobile.ts`).

## 🚀 Operational Commands

### Quick Development with dev.sh

We provide a convenient development script at `/dev.sh`:

```bash
# Backend commands (default)
./dev.sh test              # Run tests
./dev.sh format            # Format code
./dev.sh lint              # Run linter
./dev.sh build             # Build application
./dev.sh precommit         # Run all pre-commit checks

# Multiple commands (executed left to right)
./dev.sh backend format lint        # Format then lint backend
./dev.sh format lint test           # Format, lint, then test (backend default)
./dev.sh backend precommit          # Run all pre-commit checks

# Frontend commands
./dev.sh frontend test     # Run frontend tests
./dev.sh frontend build    # Build frontend

# Documentation
./dev.sh docs mermaid      # Compile Mermaid diagrams

# Help
./dev.sh help              # Show all commands
```

### Backend
```bash
# Using dev.sh (recommended)
./dev.sh test
./dev.sh format
./dev.sh lint

# Or manually
cd backend
cargo run

# Run tests
cargo test --all-features

# Add dependency
cargo add <crate_name>

# Database setup (if not running)
docker compose up --build -d db
```

### Frontend
```bash
# Using dev.sh (recommended)
./dev.sh frontend build

# Or manually
cd frontend
npm install
npm run dev
```

## 📝 Commit Message Conventions

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

> [!NOTE]
> Unlike in conventional commit, capitalize first letter in subject (unless it's an uncapitalized name).

### Types

- **build**: Changes that affect the build system or external dependencies
- **chore**: Other miscellaneous changes which do not affect users
- **ci**: Changes to our CI configuration files and scripts such as GitHub Actions
- **docs**: Documentation only changes
- **feat**: A new feature
- **fix**: A bug fix
- **perf**: A code change that improves performance
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **removal**: A code change that removes user-facing code
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- **test**: Adding missing tests or correcting existing tests
- **typing**: Changes that affect type annotations

### Scopes (Backend)

- **auth**: Authentication and authorization
- **api**: API endpoints and handlers
- **db**: Database models, tables, and queries

### Examples

```
feat(auth): Add JWT refresh token endpoint

docs(api): Update kitchen stats response schema

test(db): Add integration tests for review queries

refactor(service): Extract common pagination logic
```

## 📚 Documentation Maintenance

**Keep documentation in sync with code changes.** Whenever you make changes, update the relevant documentation:

### What to Update

- **`AGENTS.md`**: Update when changing:
  - Architecture patterns or rules
  - Development workflows or commands
  - Testing strategies
  - Security standards
  - Any instructions that affect how agents work

- **`docs/backend-development.md`**: Update when changing:
  - Setup or installation procedures
  - Development workflows
  - Testing procedures
  - Architecture patterns
  - Debugging guides

- **`docs/architecture.md`**: Update when changing:
  - System architecture
  - Database schema
  - Service interactions
  - Deployment architecture
  - Authentication flows

- **`docs/api-schema.md`**: Update when changing:
  - API endpoints
  - Request/response schemas
  - Authentication mechanisms
  - Error codes

### When to Update

Update documentation when you:
1. Add new features that change workflows
2. Modify architecture patterns
3. Change testing procedures
4. Add new environment variables or configuration
5. Change database schema
6. Add new API endpoints
7. Modify existing conventions or standards

### Documentation Commit Messages

Use `docs` type for documentation-only changes:
```
docs(agents): update testing instructions for new database setup

docs(backend): add debugging section for connection issues

docs(api): document new review batch endpoint
```

## ⚠️ Critical Rules for Agents
1.  **Security First**: Never downgrade security settings (e.g., JWT secrets, password constraints) without explicit instruction.
2.  **Test Integrity**: When modifying backend logic, always run `cargo test --all-features` to ensure no regressions.
3.  **Separation of Concerns**: Maintain the Handler -> Service -> Database boundary.
4.  **Database Access**: Services MUST use `Arc<Database>` - never access the pool directly.
5.  **Documentation**: All new functions, methods, structs, traits, and modules MUST have docstrings.
6.  **Code Quality**: Always run formatting and linting before committing:
    - `cargo +nightly fmt --all`
    - `cargo clippy --all-targets --all-features --fix --allow-dirty`
7.  **Docs Sync**: Keep AGENTS.md and all documentation updated when changing workflows, architecture, or standards.
