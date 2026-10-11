#!/bin/sh
set -eu
repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"
# Prerequisites: Rust >= 1.85, the wasm32-unknown-unknown target, and Python 3.
cargo test --locked --offline --manifest-path arena-core/Cargo.toml
cargo build --locked --offline --release --target wasm32-unknown-unknown --manifest-path arena-core/Cargo.toml
mkdir -p arena
cp arena-core/target/wasm32-unknown-unknown/release/arena_core.wasm arena/core.wasm
python3 scripts/package-arena.py
