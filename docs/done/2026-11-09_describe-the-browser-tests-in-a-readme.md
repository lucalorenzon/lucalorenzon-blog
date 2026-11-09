# Describe the browser tests in a README, kept in step by the build

- **Opened:** 2026-11-09, by the user, after the browser tests were
  reorganised by
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md).
- **Status:** done, 2026-11-09.

## Why

The browser tests are 53 tests written in fourteen files, run 228
times over pages, widths and themes. Nothing listed them in one
place: to know what protects the site from a given fault, or why a
test exists, one had to read the code. The user asked for a README in
`end2end/` that lists every test, grouped, with why it exists, what it
protects from and where it is in the code.

## Answers

All decided by the user on 2026-11-09, on the agent's questions,
unless said otherwise.

### How is a test found from its row?

**With a unique string, not a line number.** Line numbers change
whenever the code moves, and keeping them right would be endless work.
Each row carries a string that, searched with "find in files", leads
to exactly one place: the test's declaration. A Playwright test has no
function name: its title is its name. The string is a piece of the
line that declares the test, copied as the code writes it, `${...}`
parts included, that appears only once in `end2end/tests/`.

First proposed as the fixed part of the title, up to its first
variable part, and corrected by the agent on reading the tests:
`accessibility of ${title}` would have left only "accessibility of",
which three files share. Two strings of the first draft were not
unique either, and were lengthened.

### How does the README stay true?

**A project rule and a check in the build.** The rule, in the
project's `CLAUDE.md` ("The browser tests' README"): a commit that
changes a browser test updates its row, and the agent keeps the README
in step and always reports a mismatch. The check,
`end2end/tests/readme.spec.ts`, makes the report certain rather than
dependent on the agent's attention.

**Rejected:** the rule alone, which misses a change made without
reading it.

### How is the list grouped?

**First by what the tests protect from, then by ADR-004's class.**
Six sections, one per kind of fault: barriers for disabled readers,
text hard to read or to copy, layout broken on some screens, broken
navigation or pages, a wrong face to search engines and shared links,
tests that would say nothing. Inside each, one table per class.

**Rejected:** by file, since one file mixes classes; by class first,
since the reader's first question is what is protected.

### What are the classes called?

**ADR-004's three, with "the net" titled "Integration (net)",** since
"net" alone, in Italian "rete", suggests networking. The code, the ADR
and the closed cards keep "net"; the bracket links the two names. A
fourth class, **Guard**, was added by the agent for the tests that
check the suite itself (the themes guard and the README check), which
ADR-004's classes do not cover.

**Rejected:** renaming "the net" in the code's comments too.

### What does a table look like?

**A FitNesse decision table:** the columns without a question mark say
what a test runs on, the one with a question mark what makes it fail.
Columns: `Find in files`, `File`, `Pages`, `Widths`, `Themes`,
`Fails when?`, `Why`. "Find in files" was named by the user, so the
column says what it is for.

### What if one declaration serves two classes?

**Declare it twice** (the agent's proposal). The axe tests were
declared once, inside a helper of `accessibility.spec.ts` called by
both the net and the frame, so one row would have had to sit in two
tables. Each class now declares its own accessibility and contrast
tests, with the shared body in two functions.

**Rejected:** one row in the net's table with a note, the only place
where the grouping would not hold.

### How does the check work?

**A test of its own in the suite** (the agent's choice, low in impact
and easy to change), so it runs with the same command locally and in
the pipeline, with no step to remember. It asks Playwright for the
declared tests (`--list`, JSON reporter), finds each row's string in
the test files, and reports every problem at once, each with what to
fix:

- a row's string appears in the test files more than once, or never;
- its only place is not in the file the row names;
- its only place is not a line where a test is declared;
- a declared test has no row, or more than one.

It sees only the first column: whether "Fails when?" and "Why" are
still true is left to the rule. It skips on the live site, since the
README is documentation, not the site.

## Results

- The refactor of `accessibility.spec.ts` kept Playwright's list of
  tests identical, 227 titles before and after (commit `1bb3432`).
- The README lists 52 tests, one row each (commit `5685342`), and 53
  with the check's own row.
- The check fails as expected on a deleted row, on a string found
  twice and on a string found nowhere; the full suite passes, 227
  passed and the live-only test skipped (commit `625d4ea`).
- Noticed while writing the README and left as they are, now visible
  in it: `home.spec.ts` still names the two articles, their dates and
  their order, the content facts ADR-004 meant to remove; and
  `tables.spec.ts` runs on every page (in the card
  [build a design system](../backlog/build-a-design-system.md)).

Running a test from its row, the user's nice-to-have, has a card of
its own:
[run a browser test from its row in the README](../backlog/run-a-test-from-its-readme-row.md).
