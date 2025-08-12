#!/usr/bin/env bash
set -euo pipefail

# --- paths matching your tree ---
CRATE_DIR="native_rust_lib"                # contains Cargo.toml
CRATE_NAME="native_rust_lib"
HEADERS_DIR="$(pwd)/ios-rust-headers"      # should contain native_rust_lib.h AND module.modulemap
OUT_DIR="$(pwd)/ios/rust"                  # final *.xcframework goes here

# --- sanity checks ---
[ -f "$CRATE_DIR/Cargo.toml" ] || { echo "❌ $CRATE_DIR/Cargo.toml not found"; exit 1; }
[ -f "$HEADERS_DIR/native_rust_lib.h" ] || { echo "❌ $HEADERS_DIR/native_rust_lib.h not found"; exit 1; }
[ -f "$HEADERS_DIR/module.modulemap" ] || { echo "❌ $HEADERS_DIR/module.modulemap not found"; exit 1; }

# --- toolchains ---
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios >/dev/null || true

# --- build (from repo root using manifest path) ---
cargo build --manifest-path "$CRATE_DIR/Cargo.toml" --release --target aarch64-apple-ios
cargo build --manifest-path "$CRATE_DIR/Cargo.toml" --release --target aarch64-apple-ios-sim
cargo build --manifest-path "$CRATE_DIR/Cargo.toml" --release --target x86_64-apple-ios

# --- locate libs ---
LIB_DEV="$CRATE_DIR/target/aarch64-apple-ios/release/lib${CRATE_NAME}.a"
LIB_SIM_ARM64="$CRATE_DIR/target/aarch64-apple-ios-sim/release/lib${CRATE_NAME}.a"
LIB_SIM_X64="$CRATE_DIR/target/x86_64-apple-ios/release/lib${CRATE_NAME}.a"

# --- universal simulator (arm64 + x86_64) ---
UNIVERSAL_SIM_LIB="$CRATE_DIR/target/universal-sim/lib${CRATE_NAME}.a"
mkdir -p "$(dirname "$UNIVERSAL_SIM_LIB")"
lipo -create -output "$UNIVERSAL_SIM_LIB" "$LIB_SIM_ARM64" "$LIB_SIM_X64"

# --- package xcframework ---
mkdir -p "$OUT_DIR"
xcodebuild -create-xcframework \
  -library "$LIB_DEV" -headers "$HEADERS_DIR" \
  -library "$UNIVERSAL_SIM_LIB" -headers "$HEADERS_DIR" \
  -output "$OUT_DIR/${CRATE_NAME}.xcframework"

echo "✅ Created $OUT_DIR/${CRATE_NAME}.xcframework"
