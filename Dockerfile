# ---- Build Stage ----
FROM rust:1.77 as builder
WORKDIR /app

# Install system dependencies for Diesel CLI (Postgres)
RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" >src/main.rs
RUN cargo build --release && rm -rf src

# Build actual app
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim
WORKDIR /app

# Install system dependencies for Postgres
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy Diesel CLI for migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy built binary
COPY --from=builder /app/target/release/testbot /app/testbot

# Copy migrations
COPY migrations ./migrations

# Expose the web port
EXPOSE 8080

# Entrypoint: run migrations, then start the bot
ENTRYPOINT ["/bin/bash", "-c", "diesel migration run && exec /app/testbot"]
