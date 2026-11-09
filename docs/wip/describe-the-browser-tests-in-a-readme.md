# Describe the browser tests in a README, kept in step by the build

- **Opened:** 2026-11-09, by the user, after the browser tests were
  reorganised by
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md).
- **Status:** being worked on.

## Why

The browser tests are about fifty tests written in thirteen files,
run 227 times over pages, widths and themes. Nothing lists them in one
place: to know what protects the site from a given fault, or why a
test exists, one has to read the code. The user asked for a README in
`end2end/` that lists every test, grouped, with why it exists, what it
protects from and where it is in the code.

## Answers so far

All decided by the user on 2026-11-09, on the agent's questions.

- **Where a test is found: a unique string, not a line number.** Line
  numbers change whenever the code moves, and keeping them right would
  be endless work. Each row carries a string that, searched with "find
  in files", leads to exactly one place: the test's declaration. A
  Playwright test has no function name: its title is its name. The
  string is a piece of the line that declares the test, copied as the
  code writes it, `${...}` parts included, that appears only once in
  the test files. (First proposed as the fixed part of the title, up
  to its first variable part, and corrected by the agent on reading
  the tests: `accessibility of ${title}` would have left only
  "accessibility of", which three files share.)
- **Kept in step by a project rule and by a check in the build.** The
  rule, in the project's `CLAUDE.md`: the agent keeps the README up to
  date and always reports when it is not. The check makes the report
  certain rather than dependent on the agent's attention. Rejected: the
  rule alone, which misses a change made without reading it.
- **Grouped first by what the tests protect from, then by ADR-004's
  class:** one section per kind of fault (accessibility, readability,
  search engines, navigation, and so on), and inside it one table per
  class. Rejected: by file, since one file mixes classes, and by class
  first, since the reader's first question is what is protected.
- **The class ADR-004 calls "the net" is titled "Integration (net)":**
  "net" alone, in Italian "rete", suggests networking. The code, the
  ADR and the closed cards keep "net"; the bracket links the two names.
  Rejected: renaming it in the code's comments too.
- **Tables shaped as FitNesse decision tables:** the input columns say
  what a test runs on, the column with a question mark the expected
  result. Columns: `Find in files` (the unique string), `File`,
  `Pages`, `Widths`, `Themes`, `Fails when?`, `Why`.

- **The axe tests of the net and of the frame are declared apart**
  (the agent's proposal, the user's decision). They were declared once,
  inside a helper of `accessibility.spec.ts` called by both classes, so
  one declaration served two tables. Each class now declares its own
  tests, with the shared body in functions; titles and runs are
  unchanged, checked by comparing Playwright's list of tests before
  and after. Rejected: one row in the net's table with a note, the
  only place where the grouping would not hold.

## How the check works

The agent's choice, low in impact and easy to change: a test of its
own in the suite, `end2end/tests/readme.spec.ts`, so it runs with the
same command locally and in the pipeline, with no step to remember.
It asks Playwright for the list of tests (`--list`, JSON reporter),
groups them by the place they are declared, finds each row's string
in the test files, and fails when:

- a row's string appears in the test files more than once, or never;
- a row's string is not on a line where a test is declared;
- a declared test has no row in the README, or more than one.

## Not in this card

Running a single test from its README row on the last build, which
the user called a nice-to-have: a card of its own in the backlog.
