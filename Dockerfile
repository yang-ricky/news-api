FROM rustlang/rust:nightly as builder
WORKDIR /app/src
RUN USER=root cargo new --bin ht
COPY Cargo.toml Cargo.lock ./ht/

WORKDIR /app/src/ht
COPY ./ ./

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 80 443
EXPOSE 8080

COPY --from=builder /app/src/ht/target/release/news-api ./

CMD ["/app/news-api"]