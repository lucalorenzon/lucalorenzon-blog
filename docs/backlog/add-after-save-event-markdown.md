# Bring "After Save Event" into the site's Markdown

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Now.
- **Status:** noticed, not started.

## Why

The site is meant to showcase the author's experience through articles,
and its first two are already on Medium. Bringing them over was one
card, split on 2026-11-03 into one card per article's text, one for
the pages ([card](add-article-pages.md)) and one for their look
([card](design-look-of-articles.md)).

This card writes the text of "After Save Event"
(https://medium.com/@luca.lorenzon/after-save-event-905e7b9c2ff1) as a
Markdown file under `content/`. The file is not visible on the site
until the pages card is done: the user chose to prepare the text
first. What checks it in the meantime is the unit test that forbids
HTML in the content, which lists each file by name.

## Known from the start

- **The source is the text on Medium, not the author's notes.** The
  notes (`~/Notes/hybris/afterSaveEvent/afterSaveEvent.md`) hold more
  than was published, including method bodies taken from the
  decompiled platform; only what is on Medium is brought. The author
  pasted the Medium text into the session of 2026-11-03, but the
  formatting (headings, code blocks, lists, links, bold) did not
  survive the paste: it is rebuilt from the notes, which share the
  structure.
- **What the code quotes.** The configuration in `advanced.properties`,
  the fields of `AfterSaveEvent`, the `AfterSaveListener` interface,
  three lines of `core-spring.xml` and an empty outline of
  `notifyCommit` come from the platform: usage information, which
  explains the product's behaviour and can be quoted. The snippets on
  the type code are the author's. What belongs to the vendor is the
  form of its code and drawings, not the product's behaviour.
- **The comments in the configuration** are most likely the author's:
  their typos are the author's, copied from the notes. The author may
  check them on an old backup; if that is not possible, they are
  reworded.
- **The transaction diagram becomes a table** written by the author,
  one row per case: sequence of operations in one transaction → event
  generated. The eight cases: created → create; created, updated twice
  → create; updated → update; updated several times → update; updated,
  removed → remove; removed → remove; rollback → no event; created,
  updated, removed → no event. Chosen because the diagram, taken from
  the vendor's documentation, was a complex image needing a text
  description anyway, and that description is the table: one element
  instead of two, all of it the author's and under CC BY 4.0. Rejected:
  keeping the vendor's diagram with its source and a description, which
  keeps the look of the Medium version but requires checking the
  vendor's terms of use.
- **Typos and slips are corrected,** decided by the user: the site's
  copy becomes the reference version, the one search engines are to
  treat as the original. For example `core.aftersave.intervals`,
  written twice, where the key is `core.aftersave.interval`; "append"
  for "happen"; spaces lost before a word after inline code.
- **The link to the other article** becomes a Markdown link: the
  unit test forbids HTML.
- **The note in a quote** at the top keeps the Markdown blockquote; its
  look belongs to the [look card](design-look-of-articles.md).
- **The repository is public:** the text is read once more for employer
  internals before it is committed.

## Questions

- Where the file lives: `content/` directly, or a folder for articles.
- Where the link to the other article points while the article pages
  do not exist yet: its address on the site, to be decided by the
  pages card, or Medium for now.
- The licence of the code quoted from the platform: the footer will say
  that code samples are under MIT-0 "unless otherwise noted", and the
  author cannot license the vendor's code. Whether, and how, those
  blocks are marked as not under MIT-0.
