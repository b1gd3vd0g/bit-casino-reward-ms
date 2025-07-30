###############
# Build Stage #
###############
FROM rust:latest as builder
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev pkg-config
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo build --release

#################
# Runtime Stage #
#################
FROM debian:bookworm-slim
RUN useradd -m b1gd3vd0g
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/bit-casino-reward-ms .
USER b1gd3vd0g
EXPOSE 3000
CMD ["./bit-casino-reward-ms"]