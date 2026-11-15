# Update the screen reader check for an English site

- **Opened:** 2026-11-01, while the card
  [put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md)
  was being closed.
- **Status:** noticed, not started.

## Why

The site moved from Italian to English, and its page now shows the
curriculum instead of "Sito in costruzione". The on-demand screen reader
check (`scripts/screen-reader/`) still compares what Orca says with the
reference transcript of the Italian page, so its next run will report a
difference. Some of its comments, its unit tests and its log sample
also describe an Italian page.

The card
[give the screen reader container an Italian voice](give-the-screen-reader-container-an-italian-voice.md)
rested on visitors being mostly Italian; with an English site that
premise no longer holds.

- **The home page's tab title changed** on 2026-11-06, from "Home
  Page - Luca Lorenzon" to "Luca Lorenzon - Product Engineer"
  ([card](../done/2026-11-06_add-description-and-open-graph-tags.md)); the reference
  transcript and the record in `docs/accessibility/` still speak of
  "Home Page", the page's heading, which did not change.

- **Reading more pages than the home page**, added on 2026-11-15 by
  the user, while the second slice of
  [turn the code wrap toggle into an icon](../wip/turn-the-code-wrap-toggle-into-an-icon.md)
  was being closed: the check opens `/site/index.html` only, which has
  no code block, so it could not verify that card's acceptance test
  13, moved here: **the screen reader reads the wrap toggle's name
  once**, without its tooltip as a description. The user proposed the
  article `how-to-generate-hybris-pk-values-without-hybris.html`, which
  shows the toggle at the container's 1280 pixels and also has tables
  with their record viewer. Reading more pages touches the container's
  script, `check.py`, its unit tests, one reference transcript per page
  and the record in `docs/accessibility/`.

## Questions

- Which pages the check reads: the article above, the style guide's
  code page (where
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
  tests the components), or both. To be decided with the user when
  the card is taken up.

- Run the check (it needs Docker, run by the user) and accept the new
  transcript as the reference, if it reads the page as intended.
- Which comments, tests and samples in `scripts/screen-reader/` still
  describe the Italian page, and which of them should change?
- Is the Italian voice card closed as superseded, or does part of it
  still apply?
