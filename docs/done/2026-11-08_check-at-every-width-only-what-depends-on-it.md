# Check at every width only what depends on the width

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on making the browser tests check behaviour, not articles
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  second of three slices.
- **Status:** done, 2026-11-08.

## Why

Many browser tests ran at the 12 screen widths of `VIEWPORTS`
(`end2end/tests/support.ts`), on every page they check. The widths are
needed where the result can change with them, not elsewhere.

## Answers

### Which tests does the card decide?

**Every layout test, accepting named losses** (user). The card first
meant to reduce only the tests whose result cannot change with the
width; read test by test, that was one test, the record viewer's
keyboard test, because almost every layout check does depend on the
width: each table switches to the record viewer, and each code block
shows its wrap toggle, at a threshold of its own that the content sets.
The user widened the card rather than leave the widths to the card on
the net, which is wide enough already.

**Rejected:** closing the card on the keyboard test alone (the agent's
proposal), and a separate card for the layout tests' widths, which
would have carried this card's own title.

### Which widths does what is inside the text column need?

**Five: 320, 393, 412, 767 and 768** (agent's measurement, user's
decision). Measured in the browser on the hybris PK article and the
curriculum at the 12 widths:

| Screen width | Text column | Text |
|---|---|---|
| 320 to about 600 | grows with the screen, 304 to 584 pixels | 18 pixels |
| about 600 to 767 | 584 pixels | 18 pixels |
| 768 and up | 704 pixels | 20 pixels |

Table forms and code toggles follow the column. So, inside the column,
659 and 767 give the same result, and so do the seven widths from 768
to 1920; only the column's position on the screen changes. The
column's checks (readability, text spacing, content layout, home page
boxes, code toggle, table forms, record viewer rows) keep the five
widths; the table cells, checked at phone widths, keep 320, 393, 412
and 767; the record viewer's keyboard test runs at 320 only. The header
(12 widths) and accessibility (3 bands) keep theirs.

**Known limit,** as before the change: between 412 and 600, where the
column grows, the widths are sampled at three points only, so a table's
or a code block's threshold falling in between is not exercised. Reading
each threshold from the page is a question for the components' test
pages (ADR-004).

### What about the text spacing test, which also checked the header?

**Split** (agent's proposal, user's decision): every page at the five
widths for everything outside the header; the header with the reader's
spacing moved into the header's own test, at 12 widths on the two
invented titles at the limit. **On one condition** (user): the limit of
50 characters for a heading had to be enforced in the build first,
since otherwise a longer real title would have been seen by no test.
Done in its own card
([enforce the title length limit in the build](../done/2026-11-08_enforce-the-title-length-limit-in-the-build.md)).

**Rejected:** keeping the whole test at 12 widths on every page (no
loss, but the page × width product stays); moving it without the limit
enforced (a real title over the limit would go unseen).

## Results

- **Tests:** 514 before this card, 315 after (the keyboard test and the
  column's checks at fewer widths: 345; the text spacing split: 315).
  Over the session, with the first slice, 913 became 315.
- **The moved header check was seen failing:** with the title forced
  over the logo in the built stylesheet, it reported the logo link and
  the title intersecting, for each title; it passes without the fault.
  A first attempt at a fault (a negative margin on the logo) caused no
  intersection and proved nothing.
- **Pipeline** (build workflow on `main`, 2 workers):

  | Run | Tests | Sum of the tests' durations | Browser test step | Job |
  |---|---|---|---|---|
  | 42496141109, before the first slice | 911 passed | 255 s | 2.6 min | 189 s |
  | 42508735255, after the first slice | 513 passed | 145 s | 1.5 min | 127 s |
  | 42514256662, after this card | 314 passed | 136 s | 1.5 min | 121 s |

  The tests this card changed lost about 40 seconds of work
  (readability 18.5 to 9.5, table forms 16.1 to 6.7, content layout
  13.9 to 7.2, code blocks 11.2 to 7.0), about 20 seconds of wall time
  on two workers; the variance of GitHub's machines absorbed it
  (accessibility, unchanged, went from 37.3 to 51.9 seconds).
- **What bounds the time now:** the SEO check, a single test of about
  30 seconds that runs last and alone, and axe (about 37 to 52
  seconds over 75 tests), the costliest group, which the card on the
  net reworks. Locally (24 cores, 14 workers) the suite takes about 33
  seconds, set by the SEO test alone.

Commits: `0bfb789`, `ac44e7e`.
