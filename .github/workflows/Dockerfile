FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/rust-url-shortener /app/
CMD ["./rust-url-shortener"]
