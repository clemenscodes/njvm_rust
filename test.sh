#!/bin/sh

cargo fmt
cargo clippy --release --target x86_64-unknown-linux-gnu -- -D warnings