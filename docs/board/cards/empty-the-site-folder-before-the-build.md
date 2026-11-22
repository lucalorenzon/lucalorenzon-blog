# Empty the site folder before the build

- **Created:** 2026-11-22
- **Status:** Open

## Why

Asked by the user on 2026-11-22, after the card
[remove the pixel font](remove-the-pixel-font.md). The build writes the
site into `target/site` without emptying it first, so a file the site
no longer has stays there from an earlier build. That card found the
pixel font's folder still in `target/site/fonts/` after its files had
left the repository; the local browser tests and any look at the local
site then see a site that is not the one the repository describes.

## What is known

Checked by the agent on 2026-11-22.

- **Three steps write into `target/site`**, all in
  `scripts/build-site.sh`: the build program (`cargo run --release --
  target/site`, which creates the folder if missing and writes the
  pages), Tailwind (`style.css`), and a copy of `assets/` as it is
  (`cp -r assets/. target/site/`). None of them removes anything.
- **The pipeline is not affected**: each job starts from a fresh
  checkout, and `.github/workflows/deploy.yml` fills `target/site` from
  the snapshot's zip without running the build on it.
- **The build program takes its output folder as an argument**
  (`src/main.rs`, `target/site` when none is given).
- **The original photo of the logo** lies in `assets/` on the user's
  machine, excluded from git, and the copy of `assets/` puts it into
  `target/site` at every local build. Emptying the folder first does
  not change that: the photo is copied again by the same build.

## Questions to answer

The ones known when the card was written; there may be more.

- **Where the folder is emptied**: in `scripts/build-site.sh`, or in
  the build program, which knows its output folder.

  **Decided by the user on 2026-11-22.** In `scripts/build-site.sh`,
  the single entry point of the build, where the folder's path is
  fixed. The build program, which would delete whatever folder it is
  given, is left as it is; running it by hand does not empty the
  folder.
- **How the fix is proved**: a regression test that fails before the
  fix and passes after it.
- **Whether this card also keeps the original photo out of the local
  site**, or that is a card of its own.

## References

- [Remove the pixel font](remove-the-pixel-font.md): where the stale
  folder was found.
- [Make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md):
  where the original photo was put back in `assets/`.
