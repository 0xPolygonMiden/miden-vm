#!/bin/bash

# Get rust-toolchain.toml file channel
TOOLCHAIN_VERSION=$(grep 'channel' rust-toolchain.toml | sed -E 's/.*"(.*)".*/\1/')

# Get workspace Cargo.toml file rust-version
CARGO_VERSION=$(grep 'rust-version' Cargo.toml | sed -E 's/.*"(.*)".*/\1/')

# Check version match
if [ "$CARGO_VERSION" != "$TOOLCHAIN_VERSION" ]; then
    echo "Mismatch in Cargo.toml: Expected $TOOLCHAIN_VERSION, found $CARGO_VERSION"
    exit 1
fi

echo "Rust versions match ✅"
