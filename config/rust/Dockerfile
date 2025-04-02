# 1. Build stage
FROM rust:1.81 as builder
WORKDIR /app

# Skopíruj celý projekt a postav znova
COPY ./src/rust-app .
RUN cargo build --release

# 2. Run stage (čistý, malý image)
FROM rust:1.81
WORKDIR /app
COPY --from=builder /app/target/release/rust-hello-world .
EXPOSE 8080
CMD ["./rust-hello-world"]
