#!/usr/bin/env bash
# Gets the pinned Tailwind CSS standalone tool and prints its path (the only
# line on stdout; every message goes to stderr).
#
# It is a script of its own because the build (scripts/build-site.sh), the
# pipeline and a daily job that keeps the tool in the cache all need Tailwind
# without the rest of the build. It owns the pinned version and checksum.
#
# A tool already in target/tools is used, without touching the network, only if
# its checksum matches: a broken or tampered file (for example one restored from
# a cache) is removed and downloaded again. Only the pinned version is ever
# used. The download is retried, because GitHub answered 504 three times in a
# row on 2026-11-08; if it still fails, the message says it is the network.
set -euo pipefail

TAILWIND_VERSION="4.3.3"
TAILWIND_SHA256="dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a"

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# Tailwind publishes one binary per platform; only this one is pinned.
if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
    echo "error: this script supports only linux-x64 (found $(uname -s) $(uname -m))" >&2
    exit 1
fi

TOOL="target/tools/tailwindcss-${TAILWIND_VERSION}"

if [[ -e "$TOOL" ]]; then
    CACHED="$(sha256sum "$TOOL" | cut -d' ' -f1)"
    if [[ "$CACHED" == "$TAILWIND_SHA256" ]]; then
        chmod +x "$TOOL"
        echo "$TOOL"
        exit 0
    fi
    echo "Tailwind CSS ${TAILWIND_VERSION} found at ${TOOL} has the wrong checksum (expected ${TAILWIND_SHA256}, got ${CACHED}): removing it and downloading again." >&2
    rm -f "$TOOL"
fi

mkdir -p target/tools
URL="https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-linux-x64"
echo "Downloading Tailwind CSS ${TAILWIND_VERSION}..." >&2
if ! curl -fsSL --retry 6 --retry-delay 15 --retry-connrefused \
    --connect-timeout 30 --max-time 300 "$URL" -o "${TOOL}.tmp" >&2; then
    rm -f "${TOOL}.tmp"
    echo "error: could not download Tailwind CSS ${TAILWIND_VERSION} from ${URL} after several attempts. This is a network problem on the server's side, not a problem in the code: run the build again later." >&2
    exit 1
fi

ACTUAL="$(sha256sum "${TOOL}.tmp" | cut -d' ' -f1)"
if [[ "$ACTUAL" != "$TAILWIND_SHA256" ]]; then
    rm -f "${TOOL}.tmp"
    echo "error: Tailwind CSS checksum mismatch (expected ${TAILWIND_SHA256}, got ${ACTUAL})" >&2
    exit 1
fi
chmod +x "${TOOL}.tmp"
mv "${TOOL}.tmp" "$TOOL"
echo "$TOOL"
