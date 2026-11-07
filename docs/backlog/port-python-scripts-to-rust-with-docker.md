# Port the Python scripts to Rust, run with Docker

- **Opened:** 2026-11-07, by the user, right after choosing Python for
  the sitemap's dates step ([card](../wip/add-sitemap.md), "How is the
  dates step written?").
- **Status:** noticed, not started.

## Why

The user wants the project's Python scripts ported to Rust, run through
Docker.

## Known from the start

- **The Python scripts today:** the screen reader check
  (`scripts/screen-reader/check.py`, `record.py`, with their tests), run
  on demand; and the sitemap's dates step, part of every build, which
  gathers the dates of the content and will grow into the step that
  gathers content from external sources
  ([card](take-content-from-several-repositories.md)).
- **Why Rust was rejected for the dates step:** its job would have had
  to install the Rust toolchain and compile only to run `git log`,
  which makes it much slower than a script ready on the runner. Running
  a tool already compiled, from a Docker image, is a way around that
  cost.
- **The screen reader check already runs in a Docker container** of its
  own (`scripts/screen-reader/Dockerfile`), driven by the host program
  `check.py`.
- **Docker on the user's machine** is run by the user, with `sudo`,
  since the user is not in the `docker` group.

## Questions

- Why Rust for these scripts: one language for the whole build, types,
  something else; in the user's words.
- Where the Docker image is built and kept, and how its version is
  pinned, as the other tools are.
- Which script comes first.
