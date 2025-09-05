FROM rust:slim AS builder
ARG FEATURES="--features mysql,redis"
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release $FEATURES

COPY . .
RUN cargo build --release $FEATURES

FROM debian:latest
WORKDIR /app
COPY --from=builder /app/target/release/login-server-rs /usr/local/bin/login-server-rs
EXPOSE 8080 50051
ENV RUST_LOG=info
CMD ["login-server-rs"]
