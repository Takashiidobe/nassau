#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-"$root/dist"}
mkdir -p "$out"
out=$(cd "$out" && pwd)
cp -rf "$root/frontend/." "$out/"
CARGO_TARGET_DIR="$root/../target" \
CARGO_PROFILE_RELEASE_LTO=true \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
CARGO_PROFILE_RELEASE_OPT_LEVEL=s \
CARGO_PROFILE_RELEASE_STRIP=true \
    wasm-pack build "$root/.." --target web --out-dir "$out/pkg" \
    --out-name nassau --no-pack --no-typescript --no-opt \
    -- --no-default-features --features web --locked -p nassau
