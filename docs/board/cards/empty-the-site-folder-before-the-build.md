# Empty the site folder before the build

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

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

  **Answered by the user on 2026-11-22.** No automated test: the
  build script has none today, and a suite for it is not worth
  starting for one line. The only risk is deleting the wrong path, or
  something outside it. So the script names the folder written out in
  full (`target/site`), with no variable, after it has already moved
  to the repository's root; and the fix is proved once, by hand, with
  the result written in this card: a file planted in `target/site` is
  gone after the build, and the rest of `target/` is still there. This
  departs on purpose from the global rule that a fix carries a
  regression test failing before it and passing after it.
- **Whether this card also keeps the original photo out of the local
  site**, or that is a card of its own.

  **Answered by the user on 2026-11-22.** Neither: the user removed
  the photo from `assets/`, so the build no longer copies it. The copy
  an earlier build left in `target/site` is a stale file like the
  others, and goes when the folder is emptied.

## Acceptance tests

Agreed with the user on 2026-11-22. The first four are checked by hand
once, as the answer above says; the last one is the pipeline's.

1. After `scripts/build-site.sh`, `target/site/logo-original.jpg`, left
   there by an earlier build, is gone.
2. After the build, `target/generated`, `target/tools` and
   `target/release` are still there: only `target/site` is deleted.
3. In the script, the folder deleted is written out in full, with no
   variable, and is deleted after the script has moved to the
   repository's root; checked by reading the commit.
4. The build runs without errors when `target/site` does not exist, as
   in the pipeline, where every job starts from a fresh checkout.
5. Every check the pipeline runs passes: formatting, Clippy, the unit
   tests, the type check and the browser tests.

## Recap

Done on 2026-11-22: the card taken into work with its acceptance tests
(`5032daa`), and the fix in the commit that closes this card.

- **`scripts/build-site.sh`** runs `rm -rf target/site` before the
  build program writes the site, with a comment that gives the reason
  and points to this card. The path is written out in full, and the
  line comes after the script has moved to the repository's root.
- **Checked by hand**, as agreed: before the build,
  `target/site/logo-original.jpg` was there, left by an earlier build;
  after it, it was gone, and the list of the folders in `target/` was
  the same as before. A second build, with `target/site` deleted
  first, ran without errors.
- Every check of the pipeline passed: the 12 and 33 unit tests of the
  two Python steps, formatting, Clippy, the 433 unit tests, the build,
  the type check, and the browser tests, 331 passed and one skipped,
  the "page not found" checked on the live site only.

## Cautions

- **Whatever is put in `target/site` by hand is lost at the next
  build** through `scripts/build-site.sh`. Trial files belong in a
  folder of their own under `target/`, as the earlier cards did.
- **Running the build program by hand** (`cargo run -- <folder>`)
  still does not empty its folder, as decided above.

## References

- [Remove the pixel font](remove-the-pixel-font.md): where the stale
  folder was found.
- [Make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md):
  where the original photo was put back in `assets/`.
