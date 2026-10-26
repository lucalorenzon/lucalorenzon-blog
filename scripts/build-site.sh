#!/usr/bin/env bash
# Single build entry point (local and CI): generates the static site in target/site.
set -euo pipefail

TAILWIND_VERSION="4.3.3"
TAILWIND_SHA256="dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a"

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
    echo "error: this script supports only linux-x64 (found $(uname -s) $(uname -m))" >&2
    exit 1
fi

TOOL="target/tools/tailwindcss-${TAILWIND_VERSION}"

if [[ ! -x "$TOOL" ]]; then
    mkdir -p target/tools
    URL="https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-linux-x64"
    echo "Downloading Tailwind CSS ${TAILWIND_VERSION}..."
    curl -fsSL "$URL" -o "${TOOL}.tmp"
    ACTUAL="$(sha256sum "${TOOL}.tmp" | cut -d' ' -f1)"
    if [[ "$ACTUAL" != "$TAILWIND_SHA256" ]]; then
        rm -f "${TOOL}.tmp"
        echo "error: Tailwind CSS checksum mismatch (expected ${TAILWIND_SHA256}, got ${ACTUAL})" >&2
        exit 1
    fi
    chmod +x "${TOOL}.tmp"
    mv "${TOOL}.tmp" "$TOOL"
fi

cargo run --release -- target/site
"$TOOL" -i style/input.css -o target/site/style.css --minify
