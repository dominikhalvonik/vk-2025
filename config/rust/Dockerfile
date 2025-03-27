# === Development Stage ===
FROM rust:1.76-slim AS development

WORKDIR /app

# Pre rýchlejšie cacheovanie závislostí
COPY ./src/rust-app/Cargo.toml ./src/rust-app/Cargo.lock ./
RUN mkdir src && echo "fn main() {println!(\"placeholder\");}" > ./src/rust-app/main.rs
RUN cargo build

# Skutočný kód
COPY ./src/rust-app/ ./src
RUN cargo build

# === Build Stage ===
FROM rust:1.76-slim AS build

WORKDIR /app

COPY . .
RUN cargo build --release

# === Production Stage ===
FROM debian:bookworm-slim AS production
WORKDIR /app

# Na čítanie obrázkových vlastností použijeme image crate (alebo `imageinfo`)
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && apt-get clean

COPY --from=build /app/target/release/rust-app /usr/local/bin/app
CMD ["app"]
