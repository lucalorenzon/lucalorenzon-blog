# Run a browser test from its row in the README

- **Opened:** 2026-11-09, by the user, while asking for the browser
  tests' README (card
  [describe the browser tests in a README](../done/2026-11-09_describe-the-browser-tests-in-a-readme.md)),
  as a nice-to-have; in Later.
- **Status:** noticed, not started.

## Why

`end2end/README.md` lists every browser test with why it exists and
where it is. The user would like the list to be interactive: pick a
row, and that test runs on the last build, without writing the command
by hand.

## Known from the start

- **A Markdown file cannot run anything:** GitHub and the editors show
  it as text. The interaction needs a tool that reads the tests, or a
  page of its own.
- **Playwright already has an interactive mode:**
  `npx playwright test --ui` lists the tests by file and runs any one
  of them, or a group, with their trace. It does not show the README's
  grouping, why or what each protects from.
- **Today a single test runs from the command line,** with its file and
  words of its title:
  `npx playwright test tests/readability.spec.ts -g "visual presentation"`.
- **Two "last builds" exist:** the site built locally in `target/site`,
  and the tested site the pipeline keeps for each commit of `main` in
  the `snapshot` Release.

## Questions

- Which "last build": the local one, the pipeline's snapshot, or both.
- Whether Playwright's interactive mode, with a line in the README
  saying how to start it, is enough, or the README's grouping must be
  the interface itself.
- If the grouping is the interface: what it is built with (a page
  generated from the README, an editor's task, Playwright tags named
  after the groups), and how it stays in step with the README.
