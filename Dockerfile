FROM rust:1.91 AS builder
WORKDIR /app

COPY services/social/ services/social/
COPY libs/ libs/

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target \
    cargo build --release --manifest-path services/social/Cargo.toml --target-dir /app/target && \
    cp /app/target/release/social /app/social

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/social /app/social
CMD ["/app/social"]
