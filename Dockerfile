# -------------------------------------
# Base Image + Dependencies
# -------------------------------------
FROM rust:latest AS base
# Create a new empty shell project
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
# Copy the source code
COPY src ./src

# -------------------------------------
# Development Target
# -------------------------------------
FROM base AS dev
# Install watchexec
RUN cargo install watchexec-cli
# Run Watchexec
CMD ["watchexec", "--restart", "--watch", "src", "--exts", "rs", "cargo", "run"]

# -------------------------------------
# Production Target
# -------------------------------------
FROM base AS builder
# Build Application
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/axum-sqlx-repository-pattern /app/axum-sqlx-repository-pattern
EXPOSE 8000
CMD ["/app/axum-sqlx-repository-pattern"]
