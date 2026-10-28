#!/usr/bin/env bash
# Prints the @playwright/test version locked in end2end/package-lock.json.
# Used by build.yml and deploy.yml to pick the Playwright container image tag.
# The strict check also keeps anything odd out of the image name.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

version=$(jq -r '.packages["node_modules/@playwright/test"].version' end2end/package-lock.json)
if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "::error::Unexpected @playwright/test version in end2end/package-lock.json: '$version'" >&2
    exit 1
fi
echo "$version"
