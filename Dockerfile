FROM rust:1.81.0 AS build

WORKDIR /build

RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1
RUN cargo install sqlx-cli --version ^0.8
RUN apt-get update && apt-get install -y g++ valgrind vim mariadb-server
RUN cargo install cargo-valgrind
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# Prepare lock-like file
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

# Build dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Build project
ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --bin chain-chat

# Run project
FROM debian:bookworm-slim AS app

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /build/target/release/chain-chat chain-chat
COPY configuration configuration
COPY templates templates

ENTRYPOINT ["./chain-chat"]
EXPOSE 8000
