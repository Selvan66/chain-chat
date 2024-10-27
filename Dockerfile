FROM rust:1.81.0 AS base

WORKDIR /app

ARG UID=1000
ARG GID=1000

RUN  groupadd -g ${GID} builder \
    && useradd --create-home --no-log-init -u ${UID} -g "${GID}" builder \
    && groupmod -g "${GID}" builder && usermod -u "${UID}" -g "${GID}" builder \
    && chown builder:builder -R /app

USER builder

RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# DEBUG
# RUN apt-get update && apt-get install -y g++ valgrind vim mariadb-server
# RUN cargo install cargo-valgrind


# Prepare lock-like file
FROM base AS planner

USER builder

COPY --chown=builder:builder ./Cargo.lock ./Cargo.toml ./
COPY --chown=builder:builder ./src ./src

RUN --mount=type=cache,target=~/.local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder

USER builder
ENV RUST_BACKTRACE=full

COPY --chown=builder:builder --from=planner /app/recipe.json recipe.json

# Build dependencies
RUN --mount=type=cache,target=~/.local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Build project
COPY --chown=builder:builder . .

ENV SQLX_OFFLINE=true

RUN --mount=type=cache,target=~/.local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --bin chain-chat

# Run project
FROM debian:bookworm-slim AS runtime

WORKDIR /app

ARG UID=1000
ARG GID=1000

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends curl openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* /usr/share/doc /usr/share/man  \
    && groupadd -g "${GID}" run \
    && useradd --create-home --no-log-init -u "${UID}" -g "${GID}" run \
    && chown run:run -R /app

USER run

COPY --chown=run:run --from=builder /app/target/release/chain-chat chain-chat
COPY --chown=run:run configuration configuration
COPY --chown=run:run templates templates

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./chain-chat"]
EXPOSE 8000
