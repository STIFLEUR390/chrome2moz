#!/bin/bash
# Build script for WebAssembly UI
set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
SCRIPT_NAME="$(basename -- "${BASH_SOURCE[0]}")"

# --- Logging helpers ---
log_info()  { echo "[$(date +'%Y-%m-%d %H:%M:%S')] INFO:  $*" >&2; }
log_error() { echo "[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $*" >&2; }

# --- Cleanup ---
TMPDIR=""
cleanup() {
    [[ -n "$TMPDIR" && -d "$TMPDIR" ]] && rm -rf -- "$TMPDIR"
}
trap cleanup EXIT
trap 'log_error "Error on line $LINENO"' ERR

# --- Main ---
main() {
    log_info "🦊 Building Chrome to Firefox Converter WebAssembly UI"
    log_info "======================================================="

    # Check if wasm-pack is installed
    if ! command -v wasm-pack &>/dev/null; then
        log_error "❌ wasm-pack is not installed!"
        echo ""
        echo "Install it with:"
        echo "  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
        echo ""
        exit 1
    fi
    log_info "✅ wasm-pack found"

    # Build the WASM module
    log_info "📦 Building WASM module..."
    wasm-pack build --target web --out-dir web/pkg --no-default-features

    log_info "✅ WASM module built successfully!"
    echo ""
    echo "📁 Output location: ./web/pkg/"
    echo ""
    echo "🚀 To run the web UI:"
    echo ""
    echo "  Option 1 - Using Python:"
    echo "    cd web && python3 -m http.server 8080"
    echo ""
    echo "  Option 2 - Using Node.js (http-server):"
    echo "    npx http-server web -p 8080"
    echo ""
    echo "  Option 3 - Using Rust (miniserve):"
    echo "    cargo install miniserve"
    echo "    miniserve web -p 8080"
    echo ""
    echo "Then open: http://localhost:8080"
}

main "$@"
