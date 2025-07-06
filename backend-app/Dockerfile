# ---------- Base Builder Stage ----------
FROM rust:1.85.1-alpine AS base

WORKDIR /app
RUN apk add --no-cache musl-dev bash

# Copy shared files early to cache dependencies
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

# ---------- Production Build Stage ----------
FROM base AS builder

RUN cargo build --release

# ---------- Final Runtime Stage ----------
FROM alpine:3.18 AS final

RUN adduser -D appuser
USER appuser

COPY --from=builder /app/target/release/backend-app /app/backend-app

EXPOSE 5000
CMD ["/app/backend-app"]

# ---------- Dev Stage with cargo-watch ----------
FROM base AS dev

# Install cargo-watch (for hot-reloading)
RUN cargo install cargo-watch

# Use a dummy build to warm up the cache
RUN cargo build

# Mount source code during development (via docker-compose volume)
CMD ["cargo", "watch", "-x", "run"]
