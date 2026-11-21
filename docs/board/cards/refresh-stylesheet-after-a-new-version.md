# Refresh the stylesheet after a new version

- **Opened:** 2026-11-01, by the user, right after version 0.4.0 was
  published.
- **Status:** noticed, not started.

## Why

After version 0.4.0 went live, the user reloaded the page in a browser
that had already visited the site: the new text arrived, but the old
stylesheet stayed, and the page looked wrong until the browser's cache
was cleared by hand. A visitor would not know to do that.

What GitHub Pages sends today (checked on 2026-11-01 for `style.css`
and the page): `Cache-Control: max-age=600`, an `ETag` and a
`Last-Modified`. For ten minutes a browser may reuse its copy without
asking; after that it asks the server whether the file changed. GitHub
Pages does not let a site set its own headers.

## Idea to try

Give the stylesheet a name that changes when its content changes, for
example a short hash of the file in its name (`style.3f9a2c.css`) or in
a query string (`style.css?v=3f9a2c`), written into the page by the
build. A new version then asks for a file the browser has never seen.
The font and the favicon could follow the same rule.

## Questions

- Hash in the file name or in a query string: which one do browsers and
  GitHub Pages' cache handle reliably?
- Where the build computes the hash, given that the stylesheet is
  written by Tailwind after the page by the Rust program
  (`scripts/build-site.sh`).
- How a browser test proves it: the page must reference the stylesheet
  under the name the build produced.
- Is the page itself also kept for up to ten minutes, and does that
  matter?
