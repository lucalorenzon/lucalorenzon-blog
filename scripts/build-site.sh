#!/usr/bin/env bash
# Single build entry point (local and CI): generates the static site in target/site.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

TOOL="$(scripts/get-tailwind.sh)"

# The dates of the last change of each content file come from git history,
# which the build job's checkout does not have: the pipeline passes a file made
# by the gather-inputs job in CONTENT_DATES_FILE; locally it is made here.
if [[ -n "${CONTENT_DATES_FILE:-}" ]]; then
    if [[ ! -f "$CONTENT_DATES_FILE" ]]; then
        echo "error: CONTENT_DATES_FILE does not point to an existing file: ${CONTENT_DATES_FILE}" >&2
        exit 1
    fi
else
    CONTENT_DATES_FILE="target/generated/content-dates.tsv"
    python3 scripts/content-dates/content_dates.py --output "$CONTENT_DATES_FILE"
fi
export CONTENT_DATES_FILE

# The names of the robots robots.txt blocks come from a list committed in
# scripts/robots-list/ai-robots-txt/ (no download): the pipeline passes the file made
# by the gather-inputs job in BLOCKED_ROBOTS_FILE; locally it is made here.
if [[ -n "${BLOCKED_ROBOTS_FILE:-}" ]]; then
    if [[ ! -f "$BLOCKED_ROBOTS_FILE" ]]; then
        echo "error: BLOCKED_ROBOTS_FILE does not point to an existing file: ${BLOCKED_ROBOTS_FILE}" >&2
        exit 1
    fi
else
    BLOCKED_ROBOTS_FILE="target/generated/blocked-robots.txt"
    python3 scripts/robots-list/robots_list.py --output "$BLOCKED_ROBOTS_FILE"
fi
export BLOCKED_ROBOTS_FILE

# The site's full address can be set with the SITE_ADDRESS environment
# variable (https, with a host, ending with "/"); default: https://www.lucalorenzon.com/
cargo run --release -- target/site
"$TOOL" -i style/input.css -o target/site/style.css --minify
cp -r assets/. target/site/
