FROM rust:1.81.0 AS base

WORKDIR /app

RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# DEBUG
# RUN apt-get update && apt-get install -y g++ valgrind vim mariadb-server
# RUN cargo install cargo-valgrind

# Prepare lock-like file
FROM base AS prepare

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

# Build dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

FROM  debian:bookworm-slim AS prepare_run

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

###### RUNTIME #######
FROM prepare AS builder

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin chain-chat

FROM prepare_run AS runtime

COPY --chown=run:run --from=builder /app/target/release/chain-chat chain-chat
COPY --chown=run:run configuration configuration
COPY --chown=run:run templates templates
COPY --chown=run:run static static

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./chain-chat"]
EXPOSE 8000


###### TESTS ######
FROM prepare AS test_builder

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo test --release --no-run --tests
RUN rm -rf /app/target/release/deps/main-*.d

FROM prepare_run AS test_runtime

COPY --chown=run --from=test_builder /app/target/release/deps/main-* tests.bin

COPY --chown=run:run migrations migrations
COPY --chown=run:run configuration configuration
COPY --chown=run:run templates templates
COPY --chown=run:run static static

ENTRYPOINT ["./tests.bin"]
