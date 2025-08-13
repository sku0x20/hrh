#!/usr/bin/env sh

cargo build
cargo test --test e2e -- e2e "target/debug/hrh"
