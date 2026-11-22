# Update the versions Dependabot moved

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

## Why

Dependabot moved three dependencies on 2026-11-22: jiff from 0.2.43 to
1.0.1 (`052c57f`), serde-saphyr from 2.1.1 to 2.2.0 (`3f571fc`) and
`@playwright/test` from 1.65.0 to 1.65.1 (`4fff435`). The session that
merged them left an open point: the section "Stack" of `CLAUDE.md`
still named jiff 0.2 and serde-saphyr 2.1, so the project's rules
described versions the build no longer uses. The user asked to bring
every reference in line.

## What is known

Checked by the agent on 2026-11-22, outside cards and session minutes,
which describe the versions of their own time and are left as they are.

- **jiff**: Dependabot changed both `Cargo.toml` and `Cargo.lock` to
  1.0.1; only `CLAUDE.md` still said 0.2.
- **serde-saphyr**: Dependabot changed only `Cargo.lock` to 2.2.0,
  since the requirement `2.1.1` in `Cargo.toml` already allowed it;
  `CLAUDE.md` still said 2.1.
- **`@playwright/test`**: nothing to change. `end2end/package.json`
  already says 1.65.1, the test containers of both workflows take
  their image's version from the lock file through
  `scripts/playwright-version.sh`, and no document names Playwright's
  version.
- The other versions in "Stack" (Leptos 0.8.23, pulldown-cmark 0.13,
  unicode-segmentation 1.13, icondata 0.7, Lighthouse 13.6.0) match
  the lock files.

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether `Cargo.toml` raises serde-saphyr's minimum to 2.2.0**, or
  only `CLAUDE.md` changes. Raising it changes a requirement of the
  build, not only a reference, although the lock file already uses
  2.2.0.

  **Answered by the user on 2026-11-22.** Raise it, so `Cargo.toml`,
  the lock file and `CLAUDE.md` say the same version.

## Recap

Done on 2026-11-22, in the commit that writes and closes this card.

- **`CLAUDE.md`**, section "Stack": serde-saphyr 2.2 and jiff 1.0.
- **`Cargo.toml`**: serde-saphyr's minimum raised from 2.1.1 to 2.2.0.
- `cargo test --locked` passed, 433 unit tests, without changing
  `Cargo.lock`.

## Cautions

- **Dependabot changes `Cargo.toml` only when a new version falls
  outside its requirement.** A minor version that the requirement
  already allows moves the lock file alone, so `Cargo.toml` and
  `CLAUDE.md` can fall behind without any warning.

## References

- [Give dates a module of their own](give-dates-a-module-of-their-own.md):
  why jiff is used, as `CLAUDE.md` says.
- [Read article fields from front matter](read-article-fields-from-front-matter.md):
  why serde-saphyr is used, as `CLAUDE.md` says.
