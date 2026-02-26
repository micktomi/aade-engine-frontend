# Stage 1: Builder
FROM rust:1.91-slim-bookworm AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/aade-validator /app/aade-validator
COPY --from=builder /app/migrations /app/migrations

EXPOSE 3000
CMD ["/app/aade-validator"]
