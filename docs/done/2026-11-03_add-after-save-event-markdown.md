# Bring "After Save Event" into the site's Markdown

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Now.
- **Status:** done on 2026-11-03. The file is
  `content/articles/after-save-event.md`; not shown on the site until
  the card [give each article a page of its own](../backlog/add-article-pages.md).

## Why

The site is meant to showcase the author's experience through articles,
and its first two are already on Medium. Bringing them over was one
card, split on 2026-11-03 into one card per article's text, one for
the pages ([card](../backlog/add-article-pages.md)) and one for their
look ([card](../backlog/design-look-of-articles.md)). This card wrote
the text of "After Save Event"
(https://medium.com/@luca.lorenzon/after-save-event-905e7b9c2ff1) as
Markdown. The user chose to prepare the text before the pages,
knowing a file under `content/` is not visible on its own; what checks
it in the meantime is the unit test that forbids HTML in the content.

## The answers

### Which text is the source

**Chosen: the text published on Medium, not the author's notes**
(`~/Notes/hybris/afterSaveEvent/afterSaveEvent.md`). The notes turned
out to hold more than was published, including method bodies taken
from the decompiled platform; the author, three years later, did not
remember it. The author pasted the Medium text into the session; the
formatting lost in the paste was rebuilt from the notes, which share
the structure.

### What may be quoted from the platform

The article explains how a feature of SAP Commerce (hybris) behaves:
that is its point, and the behaviour of a product belongs to nobody.
What belongs to the vendor is the form of its code and of its drawings.
The code the published text quotes from the platform is usage
information, which anyone who uses the product already has: the
configuration keys and their defaults, the fields of `AfterSaveEvent`,
the `AfterSaveListener` interface, three lines of `core-spring.xml`, an
empty outline of `notifyCommit`. It stays.

### The licence of that code

The footer of an article will say that code samples are under MIT-0
"unless otherwise noted", and the author cannot license the vendor's
code. The user saw no practical risk: the quoted code is of no use
outside the platform. **Chosen: one sentence at the end of the
article**, saying that what is quoted from SAP Commerce belongs to SAP
and MIT-0 covers only the author's code; it costs one line and makes
the footer exact. Rejected: doing nothing, which leaves the footer
inexact, though harmlessly.

### The transaction diagram

**Chosen: a table written by the author** (operations on an item
within one transaction → event generated, eight rows). The diagram on
Medium comes from the vendor's documentation and, being a complex
image, needed a text description anyway; the table is that
description: one element instead of two, all of it the author's and
under CC BY 4.0. Rejected: keeping the vendor's diagram with its source
and a description, which keeps the look of the Medium version but
requires checking the vendor's terms of use.

### The comments in the configuration

Most likely the author's: their typos are the author's, copied from
the notes. The author may check them on an old backup; the two whose
wording was unclear were reworded anyway, so the text no longer
depends on that check.

### Typos and slips

**Chosen: corrected,** by the user: the site's copy is to be the
original for search engines, so it is the reference version. Among the
corrections: `core.aftersave.intervals` (twice) to
`core.aftersave.interval`; "append" to "happen"; the final list of
precautions numbered, since the text refers to "point 2"; the snippet
with `'MyItemType'` marked as Groovy, as its single quotes and bare
`return` are. "The final category" was left as written.

### Where the file lives

**Chosen: `content/articles/`,** by the user. The content folder holds
texts with different roles and different licences (the curriculum
under CC BY-ND 4.0, the articles under CC BY 4.0); a folder of their
own keeps the articles apart. The file is named after the title,
`after-save-event.md`.

### The link to the other article

**Chosen: to the article on Medium for now,** by the user, since the
other article has no address on the site yet. The pages card points it
at the site's address once there is one; the user had first proposed
the card on search-engine tags, then chose the pages card, which comes
earlier and is where the address is decided.

## Results

- `content/articles/after-save-event.md`, added to the unit test that
  forbids HTML in the content (`src/markdown.rs`): 58 tests pass,
  formatting and lint checks pass.
- Tables are not converted yet: pulldown-cmark runs with no extensions;
  switching them on is in the pages card.
