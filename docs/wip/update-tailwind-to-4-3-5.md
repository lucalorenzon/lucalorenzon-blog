# Update Tailwind to 4.3.5

- **Opened:** 2026-11-08, by the user and the agent, while checking the
  pinned tools for newer versions
  ([card](../done/2026-11-08_check-for-newer-versions-of-pinned-tools.md)).
- **Status:** being worked on, since 2026-11-08.

## Why

The site pins Tailwind 4.3.3; 4.3.5 was published on 2026-11-06. The
daily check reported it in
[issue #33](https://github.com/lucalorenzon/lucalorenzon-blog/issues/33),
the first one it opened; this update closes it.

## Known from the start

- Change `TAILWIND_VERSION` and `TAILWIND_SHA256` in
  `scripts/get-tailwind.sh`; the checksum comes from the downloaded
  file.
- The new version changes the cache's key, so the first build after it
  downloads the tool once and saves it.
- The release notes say what changed between 4.3.3 and 4.3.5: to be
  read before updating, since the stylesheet of every page comes from
  it, and the browser tests check what the pages look like.
