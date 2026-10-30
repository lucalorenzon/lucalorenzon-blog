# lucalorenzon-blog

A personal blog that showcases the author's professional experience through
articles on a range of topics.

Status: being rebuilt from scratch. The first goal is a minimal live site
(a single "under construction" home page); everything else grows from there
in small increments.

## Stack

Rust + [Leptos](https://github.com/leptos-rs/leptos), styled with
[Tailwind CSS](https://tailwindcss.com/), generated as a static site and
hosted on GitHub Pages.

## Build and test locally

Requirements: Rust (stable, installed automatically from
`rust-toolchain.toml`), Node.js 24, Python 3, Linux x64.

```sh
scripts/build-site.sh            # builds the site into target/site
cargo test                       # unit tests
cd end2end && npm ci && npx playwright install chromium
npm run typecheck                # checks the browser test's types
npx playwright test              # opens the built site in a browser
```

Every push to `main` and every pull request runs the same checks on
GitHub (workflow `.github/workflows/build.yml`). Nothing is published by
a push: the site goes live only when a version is published.

### Screen reader check, on demand

Orca 51, GNOME's screen reader, reads the built site in Firefox inside a
Docker container; the command compares what it said with the reference
transcript and records the run. It needs Docker, and asks for the `sudo`
password if your user may not use Docker directly (only the Docker commands
use `sudo`). Nothing is heard and nothing takes your keyboard: the container
has a virtual screen and a sound output that is recorded instead of played.
It is run by hand, not by the pipeline.

```sh
python3 scripts/screen-reader/check.py
python3 -m unittest discover -s scripts/screen-reader -p 'test_*.py'   # its own tests
```

The results go to `docs/accessibility/`, to be committed afterwards. If the
transcript differs from the reference, the command shows the difference and
asks whether the change is intended.

What the voice said is kept in `target/screen-reader/reading.wav`,
overwritten on every run and never committed, for when a name or a text
needs hearing:

```sh
pw-play target/screen-reader/reading.wav   # or: xdg-open target/screen-reader/reading.wav
```

The older desktop command (`python3 scripts/screen_reader_check.py`, Orca
50 on a GNOME desktop, with questions for the ear) still exists until it is
removed.

## Publish a version

A version is published from a tag, by hand. The meaning of the version
numbers and the rule on tag names are at the top of
[CHANGELOG.md](CHANGELOG.md).

1. In `CHANGELOG.md`, turn the `## [Unreleased]` heading into the
   version's own, for example `## [1.2.0] - 2026-11-03`, and add an empty
   `## [Unreleased]` above it.
2. Commit that change on `main` and push it. Wait for the build of that
   commit to pass: on `main`, a passed build saves the site it tested in
   the GitHub Release `snapshot`, and the deploy reuses it.
3. Tag that commit and push the tag:

   ```sh
   git tag v1.2.0
   git push origin v1.2.0
   ```

The tag starts the workflow `.github/workflows/deploy.yml`. It fails,
without publishing, if the tag is not of the form `vX.Y.Z`, if the commit
is not on `main`, or if `CHANGELOG.md` has no section for that version.
Otherwise it publishes the site saved in `snapshot`, if that site was
built from the tagged commit; if not (the tag came before the build
finished, or the commit changed only documentation), it builds and tests
the site from the tagged commit first. Right after publishing it creates
the GitHub Release `vX.Y.Z`, with that version's section of
`CHANGELOG.md` as its text and the zip of the published site attached;
at the same time it runs the browser test again on the public address.

## Documentation

| Document | Path | Description |
|---|---|---|
| Plan | [docs/plan.md](docs/plan.md) | What is being worked on now, next and later |
| Backlog | [docs/backlog/](docs/backlog/) | Things noticed but deliberately not done yet, with the reason |
| Work in progress | [docs/wip/](docs/wip/) | Backlog cards being worked on now |
| Done | [docs/done/](docs/done/) | Finished backlog cards, prefixed with the date they were finished |
| Session minutes | [docs/sessions/](docs/sessions/) | One record per working session: decisions and why |
| Working rules | [CLAUDE.md](CLAUDE.md) | How this project is developed |

## License

[MIT](LICENSE)
