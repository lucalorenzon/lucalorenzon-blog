# Update the screen reader check for an English site

- **Created:** 2026-11-01
- **Status:** Open

## Why

Opened on 2026-11-01, while the card
[put the curriculum on the home page](put-curriculum-on-home-page.md)
was being closed. The site moved from Italian to English, and the
on-demand screen reader check (`scripts/screen-reader/`) still compares
what Orca says with the reference transcript of the Italian page, so
its next run will report a difference. Some of its comments, its unit
tests and its log sample also describe an Italian page.

On 2026-11-15, while the second slice of
[turn the code wrap toggle into an icon](turn-the-code-wrap-toggle-into-an-icon.md)
was being closed, the user added a second reason: the check reads the
home page only, which has no code block, so it could not verify that
card's acceptance test 13, moved here: **the screen reader reads the
wrap toggle's name once**, without its tooltip as a description.

Left as it is, the check is not run, because every run would report a
difference that says nothing about the site of today.

## What is known

Brought up to date by the agent on 2026-11-22, from the built site and
the check's files.

- **The reference transcript** (`docs/accessibility/screen-reader-transcript.txt`)
  still describes the first home page, in Italian: the logo's name
  "Lvk@73r" read as text, the heading "Home Page", "Sito in costruzione"
  in the main content, the footer "© Luca Lorenzon 2026", each with the
  language `it`. The introduction of the record
  (`docs/accessibility/screen-reader-checks.md`), written again by
  `check.py` when the record is missing, still speaks of a page
  declared Italian with an English title.
- **The home page is no longer that page.** It is declared English. Its
  header holds one link, named "Lvk@73r, home page", with the logo's
  image (empty alternative text, so decorative) and the name; then the
  heading "Home Page". Its main content is the list of the articles,
  each with its title as a second-level heading and a link, its
  description and its date
  ([card](list-articles-on-home-page.md)). Its footer reads
  "© 2026 Luca Lorenzon ·" with the link "Reuse with attribution". The
  curriculum moved to a page of its own. The tab title changed on
  2026-11-06, from "Home Page - Luca Lorenzon" to "Luca Lorenzon -
  Product Engineer"
  ([card](add-description-and-open-graph-tags.md)).
- **The check reads one page.** The container's script
  (`scripts/screen-reader/entrypoint.sh`) opens `/site/index.html`
  only; `check.py` keeps one reference transcript and one record.
  Reading more pages touches the container's script, `check.py`, its
  unit tests, one reference transcript per page and the record.
- **Where a code block is.** On 2026-11-15 the user proposed the
  article `how-to-generate-hybris-pk-values-without-hybris.html`, which
  shows the toggle at the container's 1280 pixels and also has tables
  with their record viewer. Since then the style guide was built, and
  [ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
  tests each component once, on its page there: the code block on
  `style-guide/code.html`
  ([card](show-the-code-block-in-the-style-guide.md)), the page frame
  on `style-guide/frame.html`
  ([card](show-the-page-frame-in-the-style-guide.md)). On 2026-11-22
  the frame's page has no code block; the code page has seven.
- **The Italian voice** asked by
  [give the screen reader container an Italian voice](give-the-screen-reader-container-an-italian-voice.md)
  rested on the visitors being mostly Italian; with an English site
  that premise no longer holds.
- **The check needs Docker**, run by the user in their own terminal.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which pages the check reads.** The home page, the article above,
  the style guide's pages, or a mix of them.

  **The user's view on 2026-11-22, not a decision yet.** Reading the
  home page makes little sense; the page frame's page in the style
  guide probably makes more. Still open: that page has no code block,
  so the wrap toggle's acceptance test would need the code page too,
  or a different place.
- **How the new references are accepted**: a run of the check by the
  user, and the agent's reading of the new transcript against what the
  page is meant to say.
- **Which comments, tests and samples in `scripts/screen-reader/`
  still describe the Italian page**, and which of them should change.
- **Whether the Italian voice card is closed as superseded**, or part
  of it still applies.

## Dependencies

- A run of the check with Docker, by the user.

## References

- [Put the curriculum on the home page](put-curriculum-on-home-page.md):
  the card being closed when this one was opened.
- [List the articles on the home page](list-articles-on-home-page.md):
  what the home page shows today.
- [Add description and Open Graph tags](add-description-and-open-graph-tags.md):
  the change of the home page's tab title.
- [Turn the code wrap toggle into an icon](turn-the-code-wrap-toggle-into-an-icon.md):
  the acceptance test moved here.
- [Show the code block in the style guide](show-the-code-block-in-the-style-guide.md)
  and [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the pages the check could read.
- [Give the screen reader container an Italian voice](give-the-screen-reader-container-an-italian-voice.md):
  the card this one decides on.
- [Run a screen reader check on demand](run-screen-reader-check-on-demand.md)
  and [run it in a container](run-screen-reader-check-in-a-container.md):
  how the check came to be.
