# Measure the CO2 of a page view

- **Opened:** 2026-11-02, by the user, together with the card
  [check performance in the build](check-performance-in-the-build.md).
- **Status:** noticed, not started; in Later, as the user asked.

## Why

The user would like a measure of the carbon dioxide emitted by a visit
to the site, alongside accessibility and performance, and possibly
shown as a badge (card
[show the check results as badges in the README](show-check-results-as-readme-badges.md)).

## Known from the start

- **It is an estimate, not a measurement.** The usual tools, such as
  CO2.js by the Green Web Foundation (open source) or the Website Carbon
  Calculator, estimate it from the bytes transferred, with a model of
  the energy used by networks, data centres and devices, and whether
  the hosting runs on renewable energy.
- **The bytes are the lever** the site controls: the same budget of
  sizes the performance card may introduce would drive this estimate
  too.

## Questions

- Which tool and which model of estimate.
- Whether it runs in the build, on the live site, or both, and whether
  it fails the build above a threshold or only reports.
- How the hosting (GitHub Pages) is counted: green or not, and on what
  evidence.
