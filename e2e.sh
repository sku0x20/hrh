#!/usr/bin/env sh

cargo build
cargo test --test e2e -- --test-threads=1 --nocapture
