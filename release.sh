#!/bin/bash

cargo build --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
cargo publish