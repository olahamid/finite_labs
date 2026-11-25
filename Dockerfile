# ---- build stage: compile the server and the WebAssembly demos -------------
FROM rust:1.99-slim-trixie AS builder
WORKDIR /src

# Tooling for the browser demos (cached layer). The CLI version must match the
# wasm-bindgen crate version pinned in Cargo.toml.
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli --version 0.2.130 --locked

# Build dependencies first (cached layer) using stub sources, so editing app
# code doesn't recompile every crate. rust-toolchain.toml is deliberately not
# copied: the image's own toolchain is used instead of downloading another.
COPY Cargo.toml Cargo.lock ./
COPY crates/web/Cargo.toml crates/web/askama.toml crates/web/
COPY crates/crypto/Cargo.toml crates/crypto/
COPY crates/playground/Cargo.toml crates/playground/
RUN mkdir -p crates/web/src crates/crypto/src crates/playground/src \
    && echo "fn main() {}" > crates/web/src/main.rs \
    && touch crates/web/src/lib.rs crates/crypto/src/lib.rs crates/playground/src/lib.rs \
    && cargo build --release -p web \
    && cargo build --release -p playground --target wasm32-unknown-unknown \
    && rm -rf crates/*/src

# Real sources. Templates are compiled into the server binary by Askama.
COPY crates crates
COPY templates templates
RUN find crates -name '*.rs' -exec touch {} + \
    && cargo build --release -p web \
    && cargo build --release -p playground --target wasm32-unknown-unknown \
    && wasm-bindgen --target web --no-typescript --out-dir /src/pkg \
       target/wasm32-unknown-unknown/release/playground.wasm

# ---- runtime stage: small image with only what we need ---------------------
FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home app

WORKDIR /app
COPY --from=builder /src/target/release/web /app/web
COPY data data
COPY static static
COPY --from=builder /src/pkg static/pkg

USER app
ENV PORT=3000
EXPOSE 3000
CMD ["/app/web"]
