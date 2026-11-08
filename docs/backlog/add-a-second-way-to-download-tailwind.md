# Add a second way to download Tailwind

- **Opened:** 2026-11-08, by the user and the agent, while making the
  build's external downloads reliable
  ([card](../wip/make-external-downloads-in-the-build-reliable.md)).
- **Status:** noticed, not started.

## Why

Since 2026-11-08 the pipeline keeps the pinned Tailwind in GitHub's
cache, a daily job keeps the cache alive, and the download is tried
again before giving up. The download from GitHub's release server is
then needed only when the cache is empty: once per new version, or when
GitHub loses the cache for reasons of its own. If the release server
fails for a long time in one of those moments, as it did on 2026-11-08,
the build still stops.

On that day the same file arrived whole through GitHub's programming
interface, while the release address answered `504`.

## Known from the start

- The address is
  `api.github.com/repos/tailwindlabs/tailwindcss/releases/assets/<id>`,
  with the header `Accept: application/octet-stream`. The asset's
  number changes with each version, so it would have to be pinned too,
  or looked up.
- Without a token, the interface allows 60 requests an hour per
  address, and the pipeline's machines are shared: in the pipeline the
  token GitHub Actions provides would be needed, passed to
  `scripts/get-tailwind.sh`.
- The checksum check stays the same whichever way the file arrives.

## When to pick this up

If the build stops again on Tailwind's download although the cache and
the retries are in place. Until then it is code for a case seen once.
