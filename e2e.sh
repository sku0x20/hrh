#!/usr/bin/env sh

cargo build
cargo test --test e2e -- release diff --nocapture "target/debug/hrh"
