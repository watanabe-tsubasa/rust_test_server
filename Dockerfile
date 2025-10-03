# syntax=docker/dockerfile:1

# -------- Builder stage --------
FROM rust:1.80-bullseye AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# Build release binary
RUN cargo build --release

# -------- Runtime stage --------
# Distroless for minimal attack surface
FROM gcr.io/distroless/cc-debian12
WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/rust_test_server /app/server

# Cloud Run provides $PORT (default 8080). Our app reads it.
ENV PORT=8080

# Optional: set a non-root user (distroless nonroot user id)
USER 65532:65532

# Serve
CMD ["/app/server"]
