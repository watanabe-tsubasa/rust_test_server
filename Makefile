SHELL := /bin/sh

.PHONY: build run fmt clippy migrate

build:
	cargo build

run:
	cargo run

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

migrate:
	psql "$$DATABASE_URL" -f migrations/20251003_create_todos_postgres.sql
