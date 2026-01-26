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

1.  **Handlers (`src/**/handlers.rs`)**:
    - Entry points for HTTP requests.
    - Responsible for parsing input (JSON), validating simple constraints, and calling the Service layer.
    - **Rule**: Do NOT put business logic or database queries here.

2.  **Service Layer (`src/service/`)**:
    - Contains business logic.
    - Orchestrates database operations and external services.
    - Example: `AuthService` handles hashing passwords and generating tokens.

3.  **Database Layer (`src/database/`)**:
    - **Models (`src/database/model.rs`)**: Structs representing DB tables.
    - **Tables (`src/database/table.rs`)**: Data access objects using the `Table` trait.
    - **Rule**: Use `sqlx` for type-safe queries. Extend specific Table structs (e.g., `UserTable`) for custom queries like `find_by_unique_code`.

### Security Standards
- **Passwords**: Must be 8-32 characters. Always hash using `AuthService`.
- **Input**: Sanitize strings (trim) in handlers. Validate lengths.
- **Timing Attacks**: Use constant-time comparisons or dummy verification logic in login flows.
- **JWT**: Short expiration (1 hour).

### Testing Strategy
- **Unit Tests**: Place inside the module (e.g., `src/auth/utils.rs`).
- **Integration Tests (`tests/`)**:
    - Use `common::setup_db()` to create a fresh, isolated database for each test.
    - Use `common::teardown_db()` to clean up.
    - Test full HTTP flows using `tower::ServiceExt::oneshot`.

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

### Backend
```bash
# Run locally
cd backend
cargo run

# Run tests
cargo test

# Add dependency
cargo add <crate_name>
```

### Frontend
```bash
# Install dependencies
cd frontend
npm install

# Run development server
npm run dev
```

## ⚠️ Critical Rules for Agents
1.  **Security First**: Never downgrade security settings (e.g., JWT secrets, password constraints) without explicit instruction.
2.  **Test Integrity**: When modifying backend logic, always run `cargo test` to ensure no regressions.
3.  **Separation of Concerns**: Maintain the Handler -> Service -> Database boundary.
