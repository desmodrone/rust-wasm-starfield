#!/usr/bin/env bash
set -euo pipefail

TARGET="wasm32-unknown-unknown"
PROJECT_NAME="rust-wasm-starfield"
OUT_DIR="web"
PORT=4000

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_PATH="$ROOT_DIR/target/$TARGET/debug/$PROJECT_NAME.wasm"
DEST_PATH="$ROOT_DIR/$OUT_DIR/$PROJECT_NAME.wasm"

echo "Building $PROJECT_NAME for WebAssembly..."
cargo build --target "$TARGET"

echo "Copying wasm into $OUT_DIR/ ..."
mkdir -p "$OUT_DIR"
cp "$WASM_PATH" "$DEST_PATH"

echo "Build complete: $DEST_PATH"

if command -v basic-http-server >/dev/null 2>&1; then
    if lsof -i :"$PORT" >/dev/null 2>&1; then
        echo "Port $PORT is already in use."
        echo "If your local server is already running, just refresh the browser."
        echo "Otherwise, stop the existing process or change PORT in build.sh."
        exit 0
    fi
    echo "Serving $OUT_DIR at http://127.0.0.1:$PORT"
    basic-http-server "$OUT_DIR" -a 127.0.0.1:$PORT
else
    echo "basic-http-server not found. Install it with:"
    echo "  cargo install basic-http-server"
fi
