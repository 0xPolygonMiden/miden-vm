#!/bin/bash
cargo clippy --workspace --all-targets --all-features \
  -W clippy::pedantic \
  -W clippy::style \
  -W clippy::missing_docs \
  -- -D warnings
