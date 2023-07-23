FROM rust:1.68 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release

FROM rust:1.68
COPY --from=builder /app/target/release/rust_server /bin/rust_server
ENTRYPOINT ["rust_server"]
