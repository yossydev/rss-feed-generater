FROM rust:1.80.1 as builder
WORKDIR /usr/src/rss_feed_generator
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y openssl libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rss_feed_generator/target/release/rss_feed_generator_bin /usr/local/bin/rss_feed_generator
EXPOSE 8080
CMD ["rss_feed_generator"]
