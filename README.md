# Rust Test Server (Axum + SQLx)

A minimal REST API built with Axum and SQLx. It provides a simple Todo CRUD and a demo Users endpoint, using Postgres (Neon-ready) and automatic SQL migrations at startup.

## Quick Start

1) Prerequisites
- Rust (stable) and Cargo
- A Postgres database (Neon is supported)

2) Configure environment
- Create a `.env` with your connection string:
  - `DATABASE_URL=postgresql://<user>:<pass>@<host>/<db>?sslmode=require&channel_binding=require`
  - Optional: use `PGHOST/PGUSER/PGPASSWORD/PGDATABASE` instead of `DATABASE_URL`.
- Optional: `PORT=3000` (default 3000)

3) Run
- `cargo run`
- On start, the app runs migrations from `migrations/` and listens on `0.0.0.0:$PORT`.

## API Endpoints

- `GET /` — Healthy hello.
- `GET /healthz/db` — DB health check (204 on success).
- `POST /users` — Create a user (demo endpoint).
- `GET /todos` — List todos.
- `POST /todos` — Create todo. Body: `{ "title": "..." }`
- `GET /todos/:id` — Get a todo by id.
- `PUT /todos/:id` — Update done flag. Body: `{ "done": true }`
- `DELETE /todos/:id` — Delete a todo by id.

Examples
- `curl -s :3000/healthz/db -i`
- `curl -s -X POST :3000/todos -H 'content-type: application/json' -d '{"title":"test"}'`
- `curl -s :3000/todos`
- `curl -s :3000/todos/1`
- `curl -s -X PUT :3000/todos/1 -H 'content-type: application/json' -d '{"done":true}'`

## Development

- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Build: `cargo build`
- Run: `cargo run`
- Migrations: applied automatically on startup from `migrations/`. To run manually: `psql "$DATABASE_URL" -f migrations/<file>.sql`

Project layout
- `src/main.rs` — Entrypoint, router, server.
- `src/handlers/` — Request handlers (`/todos`, `/users`, `/healthz/db`).
- `src/models.rs` — DTOs and SQLx row structs.
- `src/db.rs` — Postgres pool + startup migrations.
- `migrations/` — SQL migrations.
- `Dockerfile` — Multi-stage build (distroless runtime). Uses `$PORT`.

## Docker / Cloud Run

- Build: `docker build -t rust-test-server .`
- Run: `docker run -e PORT=8080 -e DATABASE_URL=... -p 8080:8080 rust-test-server`
- Cloud Run (example):
  - `gcloud builds submit --tag gcr.io/<PROJECT_ID>/rust-test-server:latest`
  - `gcloud run deploy rust-test-server --image gcr.io/<PROJECT_ID>/rust-test-server:latest --platform managed --region <REGION> --allow-unauthenticated`
  - Set `DATABASE_URL` in the service environment.

## Notes

- Do not commit real secrets. Prefer Cloud Run service-level env vars.
- Queries use bind parameters; Postgres-only. SQLite is not supported in this branch.
