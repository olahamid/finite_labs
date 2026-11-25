# Finite Labs

Finite Labs builds real-world cryptographic solutions, through software, hardware, education, and research, to put ownership and control back in your hands.

This repository is the Finite Labs website: a server-rendered Rust app (Axum + Askama). See [PRD.md](PRD.md) for the full plan. **M1: Foundation** is done; part of **M3** (the Live Proof widget) was brought forward.

## Run locally

Run everything from the repository root (the server reads `data/` and `static/` relative to it).

```bash
# one-time setup for the browser demos (Rust → WebAssembly)
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked

./scripts/build-wasm.sh   # builds crates/playground into static/pkg/ (re-run after changing it)
cargo run -p web          # http://localhost:3000
PORT=8080 cargo run -p web
```

The site works without the WebAssembly build; the homepage Live Proof widget just stays in its "needs WebAssembly" state.

## Test and lint

```bash
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check          # needs: cargo install cargo-deny
```

## Docker

```bash
docker build -t finite-labs .
docker run --rm -p 3000:3000 finite-labs
```

## Layout

| Path | What |
|---|---|
| `crates/web/` | The Axum server (`lib.rs` builds the router, `main.rs` starts it) |
| `crates/crypto/` | Thin wrappers around audited crypto crates (`sha2`, `ed25519-dalek`), tested against NIST / RFC 8032 vectors |
| `crates/playground/` | Browser demos in Rust, compiled to WebAssembly (the homepage Live Proof widget) |
| `scripts/build-wasm.sh` | Builds `crates/playground` into `static/pkg/` |
| `templates/` | Askama templates, compiled into the binary |
| `data/site.toml` | All site copy: nav, homepage sections, footer, about page |
| `static/` | CSS, the theme toggle script, the WebAssembly loader, self-hosted fonts, favicon |
| `render.yaml` | Render deployment blueprint |

## Editing site text

Change `data/site.toml` and restart the server. Homepage sections render in the order they appear in the file. Unknown keys are rejected at startup, so typos fail loudly.
