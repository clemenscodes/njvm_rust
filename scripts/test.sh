#!/bin/sh

TARGET="x86_64-unknown-linux-gnu"

cargo fmt --all -- --check
cargo clippy --release --target $TARGET -- -D warnings
cargo test --workspace --release --target $TARGET
cargo build --release --target $TARGET --locked

cp target/x86_64-unknown-linux-gnu/release/njvm njvm
