FROM rust:1.81.0 AS base

RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# DEBUG
# RUN apt-get update && apt-get install -y g++ valgrind vim mariadb-server
# RUN cargo install cargo-valgrind


# Prepare lock-like file
FROM base AS planner

WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Build project
COPY . .

ENV SQLX_OFFLINE=true

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --bin chain-chat

# Run project
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends curl openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/chain-chat chain-chat
COPY configuration configuration
COPY templates templates

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./chain-chat"]
EXPOSE 8000
