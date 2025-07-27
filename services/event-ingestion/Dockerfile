FROM rust:latest as builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/metricbox-event-ingestion /app/metricbox-event-ingestion

EXPOSE 8080

CMD ["./metricbox-event-ingestion"]
