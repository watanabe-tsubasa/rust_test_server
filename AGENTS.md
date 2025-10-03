# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: Axum app entrypoint and router setup.
- `src/handlers/`: Request handlers (e.g., todo CRUD) re-exported via `mod.rs`.
- `src/models.rs`: Request/response DTOs and DB row structs.
- `src/db.rs`: Database pool initialization (Postgres-only, Neon-ready TLS).
- `migrations/`: SQL migration files (e.g., create `todos` table).
- `Cargo.toml`: Features and dependencies. `.env`: local configuration.

## Build, Test, and Development Commands
- Build: `cargo build`
- Run locally (listens on `0.0.0.0:3000`): `cargo run`
- Format and lint:
  - `cargo fmt` (or `cargo fmt -- --check` in CI)
  - `cargo clippy -- -D warnings`

## Configuration & Migrations
- Env is loaded via `dotenvy`. Use `.env` with either:
  - `DATABASE_URL=postgres://...` (TLS enforced in code), or
  - `PGHOST/PGUSER/PGPASSWORD/PGDATABASE` (TLS required for Neon).
- Apply migrations:
  - `psql "$DATABASE_URL" -f migrations/<timestamp>_create_todos_postgres.sql`
  - Alternatively, use `sqlx migrate run` if using `sqlx-cli` (optional).

## Coding Style & Naming
- Rust 2021; 4-space indentation; `snake_case` for modules/functions, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for consts.
- Keep handlers small; put shared types in `models`, DB access via `db` and `sqlx` queries.
- Run `cargo fmt` and `cargo clippy` before pushing.

## Testing Guidelines
- Use `cargo test`. Place integration tests in `tests/` and unit tests in `#[cfg(test)]` modules.
- Name tests descriptively (e.g., `creates_todo_returns_201`).
- Prefer testing handler behavior via Axum router + state.

## Commit & Pull Request Guidelines
- Commits: short, imperative subject (max ~72 chars), e.g., "add todo update endpoint"; group related changes.
- PRs: include what/why, run steps, and screenshots or `curl` examples for new endpoints.
- Link related issues; keep PRs focused; update docs/migrations as needed.

## Security Tips
- Do not commit real secrets or local DB files; use `.env` and ignore artifacts.
- Prefer TLS-enabled Postgres (`PGSSLMODE=require`).
