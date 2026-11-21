# Show the check results as badges in the README

- **Opened:** 2026-11-02, by the user, together with the card
  [check performance in the build](check-performance-in-the-build.md).
- **Status:** noticed, not started; comes after that card.

## Why

The build checks the WCAG criteria (axe, the accessibility tree, the
readability test for 1.4.8 and 1.4.12) and will check performance. The
user would like their results shown as badges from shields.io in the
repository's `README.md`, so the state of the site is visible at a
glance.

## Known from the start

- **shields.io** draws a badge from a fixed text, from a JSON file it
  reads at a public address (its "endpoint" badge), or from services it
  knows, GitHub Actions' workflow status among them.
- **A workflow status badge** says only that the build passed, not
  which criteria or scores; detailed badges need the results published
  somewhere shields.io can read them, for example with the site or in
  the GitHub Release.
- **What a badge claims must be true.** A badge "WCAG AAA" would claim
  more than the build checks: many criteria are judged only by a person
  (card [cover the WCAG criteria axe cannot check](cover-wcag-criteria-axe-cannot-check.md)).

## Questions

- Which badges: build status, accessibility checks, performance scores,
  later the CO2 estimate.
- Where the results are published for shields.io to read, and when they
  are refreshed (each build of `main`, or each publication).
- How each badge's wording stays true to what is actually checked.
