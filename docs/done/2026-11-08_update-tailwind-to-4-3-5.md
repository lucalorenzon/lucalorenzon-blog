# Update Tailwind to 4.3.5

- **Opened:** 2026-11-08, by the user and the agent, while checking the
  pinned tools for newer versions
  ([card](2026-11-08_check-for-newer-versions-of-pinned-tools.md)).
- **Status:** done, on 2026-11-08.

## Why

The site pinned Tailwind 4.3.3; 4.3.5 was published on 2026-11-06. The
daily check reported it in
[issue #33](https://github.com/lucalorenzon/lucalorenzon-blog/issues/33),
the first one it opened. Every page's stylesheet comes from Tailwind, so
the update had to show what it changes before being made.

## Answers

### What changed between 4.3.3 and 4.3.5?

Only fixes, in the release notes of
[4.3.4](https://github.com/tailwindlabs/tailwindcss/releases/tag/v4.3.4)
and
[4.3.5](https://github.com/tailwindlabs/tailwindcss/releases/tag/v4.3.5).
Three of them could touch the site, and were checked against the
stylesheet: container query variants with decimal values are now sorted
by number, and the site gives every code block and table one
(`@max-[Wem]/code`, `@max-[Tem]/table`); the fallback for older Safari
versions changed its test; empty `var()` fallbacks get a space.

### Does the site look different?

No. The stylesheet was built from the same input with both versions and
compared. It differs in three places only: the version in its header;
the order of three code block thresholds, now sorted by number, which
changes nothing because each block has one threshold and each rule has
a selector of its own; and the Safari fallback's test,
`margin-trim: block` instead of `margin-trim: inline`, which concerns
only Tailwind's own variables in Safari. The minified file is one byte
shorter. No screenshots were taken, since nothing visible changed.

### How is the checksum trusted?

It was computed on the downloaded file and compared with the line for
`tailwindcss-linux-x64` in the release's own `sha256sums.txt`: the two
match. So the pin rests on two sources, not only on the download.

## Results

Commit `3250a95` changes `TAILWIND_VERSION` and `TAILWIND_SHA256` in
`scripts/get-tailwind.sh`, and nothing else; its message closes issue
#33 once it reaches GitHub. All checks passed locally, as in the
pipeline: format, clippy, 227 unit tests, the build (which downloaded
4.3.5), the type check, and the browser tests (911 passed, 2 skipped on
purpose in a local run). The cache's key changes with the version, so
the first pipeline run after the push downloads the tool once and saves
it.
