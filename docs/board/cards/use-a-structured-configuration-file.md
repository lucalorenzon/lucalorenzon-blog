# Use a structured configuration file shared by the build and the tests

- **Opened:** 2026-11-08, by the user, while deciding where the limit of
  a page heading lives, in the card
  [enforce the title length limit in the build](enforce-the-title-length-limit-in-the-build.md);
  in Later.
- **Moved:** 2026-11-16, by the user, to Next, now that `config/`
  holds a second setting, the fonts (`config/fonts.txt` and
  `config/fonts/`), although only the build reads them.
- **Status:** noticed, not started.

## Why

The limit of a page heading lives in `config/heading-max-graphemes.txt`,
a file holding only the number, read by the build when it is compiled
and by the header's browser test from the disk. The user had meant a
structured configuration file, read by both Rust and JavaScript, such
as a section of `Cargo.toml` or a `config.toml`; with a single number
the plain file was kept for now. Once there is more than one setting,
one file per number stops scaling.

## Known from the start

- **`Cargo.toml`** does not pass its metadata (`[package.metadata]`) to
  the code, so the build would need a build script to read it.
- **TOML or YAML** need a parser on the tests' side; Rust already reads
  YAML with `serde-saphyr`. **JSON** is read by JavaScript with nothing
  added, and by Rust with a crate.
- The browser tests read the file from the disk, also on the live
  check, since they always run from a copy of the repository.
