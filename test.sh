#!/bin/sh

cargo fmt
cargo clippy --release --target x86_64-unknown-linux-gnu -- -D warnings
cargo test --workspace --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu --locked
