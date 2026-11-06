# List the articles on the home page

- **Opened:** 2026-11-06, by the user, splitting the card on site
  navigation, opened on 2026-10-29; the other part is the card
  [add a menu for the site](../backlog/add-site-menu.md). In Now.
- **Status:** done on 2026-11-06. The home page lists the two
  articles, newest first, and the curriculum has its own page,
  `curriculum.html`. Validated by the user in the browser. A version is
  published next, the articles with it.

## Why

The two article pages are built, but no page links to them, and no
version is published until a reader can find them
([card](../done/2026-11-04_add-article-pages.md)). The pages are now
accessible and in as good a state as the curriculum, validated by the
user on 2026-11-06 ([card](../done/2026-11-06_design-look-of-articles.md)).

Asked what the first published navigation must let a reader do, the
user answered: find and read the articles, the rest later. Search,
categories, subscribing and settings stay with the
[menu card](../backlog/add-site-menu.md). When this card is done, a version is
published, the articles with it.

## The answers (2026-11-06)

### Where the list lives

**Chosen, by the user: on the home page,** so a reader arriving on the
site sees the articles first; the curriculum moves to a page of its
own. This was already the intent of the card on site navigation. The
other way, the list on a page of its own, would keep the home page as
it is, but a reader who came for the articles would see the curriculum
first and have to look for them.

### How a reader gets there

**Chosen, by the user: with what the site already has.** The logo on
every page leads to the home page, the list there leads to the
articles, the footer's link leads to the about page, and that page
leads to the curriculum. The right side of the header stays empty for
now: a visible tool there is the [menu card](../backlog/add-site-menu.md).

### The curriculum's address

**Chosen, by the user: `curriculum.html`, at the site's root,** beside
`about.html` and `licence.html`, named by its subject as they are. An
address, once published, ends up in shared links and search engines,
and GitHub Pages does not make redirects easy, so it is meant to stay.
The [card on the curriculum in more languages](../backlog/publish-curriculum-in-more-languages.md),
in Later, will find its own addresses beside it: the user left the
languages for then. Rejected: an address laid out for languages
today, which would prepare something that may never come.

### Readers who kept the old address

The curriculum has been at the site's root, `www.lucalorenzon.com/`,
since version 0.4.0. A reader who had that address as the link to the
curriculum would land on the list and might need a visible way to it
from the home page. **The user has not given that address to anyone,
and will not promote the site** until it has observability, search
engine tags and a few other features
([observe the visitors](../backlog/observe-visitors-in-production.md),
[SEO and Open Graph tags](../backlog/add-seo-and-open-graph-tags.md)).
So the way to the curriculum chosen above is enough: the footer's link
to the about page, and the about page's link to the curriculum.

### What each entry shows

**Chosen, by the user: the title, a one-line description and a date,
written in the code for now, as the title already is.** The title is
the only thing the site knew of an article: it lives in the code, not
in the Markdown. A date, a summary or a category would need fields of
each article's own, and the model of an "article" was left to the
[run of `residuality`](../backlog/run-full-residuality-round.md) and to
the [card on search-engine tags](../backlog/add-seo-and-open-graph-tags.md),
which plans fields each article declares. The user first chose the
title only; offered a description written in the code as an
alternative, which gives the reader something to choose by without
waiting for that model, the user took it, and added the date. The
cost: more text in the code, to move when the articles get fields of
their own.

### The order of the list

**Chosen, by the user: by date.**

### Which date

Each article has two dates: the day it was published on Medium, and the
day it is published on this site, the next version's day, the same for
both, so it could not order them. **Chosen, by the user, for these two
articles: the date of Medium, the day they were made public:** "How to
generate hybris PK values without hybris" on 25 April 2023, "After Save
Event" on 30 April 2023. The later one links to the earlier, as it did on
Medium. An article first published on this site will take this site's
date.

### Newest or oldest first

**Chosen, by the user: newest first,** so a returning reader finds the
latest article at the top. The cost, with today's two articles: "After
Save Event" comes before the article it links to. Rejected: oldest
first, which reads the two in the order they were written but puts
every new article at the bottom.

### The descriptions

**Written by the user,** with three touches the agent pointed out and
the user accepted: "SAP hybris", as the articles write it; "inner
behaviour"; "Dive into" and "use cases".

| Article | Date | Description |
|---|---|---|
| After Save Event | 30 April 2023 | SAP hybris change events system: architecture and inner behaviour. |
| How to generate hybris PK values without hybris | 25 April 2023 | Dive into the PK generation logic of SAP hybris, for extreme use cases. |

### The home page's title

The shared frame gives every page one title, used for the browser tab
("<page> - Luca Lorenzon") and as the page's only `h1`, the first thing
a screen reader announces. "Curriculum Vitae" no longer fits.
**Chosen, by the user: "Home Page",** the title the home page had
before the curriculum came (version 0.2.0): general enough to last when
the menu, categories or other kinds of content arrive. Not chosen: a
title naming the list, such as "Articles", which tells an arriving
reader more but would change when the home page holds more than
articles.

### The look of the list

**Chosen, by the user: each entry with the title first, as a link,
then the description, then the date written out ("30 April 2023");**
no bullets and no indent; title, description and date close together,
entries apart like paragraphs; the title in the site's section-title
style. The user compared, on the built page at iPhone SE width on both
backgrounds, and at laptop width:

- **The date above the title.** Not chosen: it shows at once when an
  article came out, but the title no longer opens the entry; it suits
  a list where dates matter more than subjects, such as a diary.
- **The date written as "2023-04-30".** Not chosen: shorter and the
  same in every language, but it reads as a technical record.

Decided by the agent, as it changes nothing a reader sees: the list is
an HTML list, and each title is a heading, so a screen reader can move
from article to article; the order comes from the dates in the code,
so a new article takes its place by itself. The date is a `<time>`
element rather than a paragraph, which keeps it close to the
description: the readability check asks 1.5 lines between two
paragraphs. The first build still showed the text column's bullets
and indent, whose rule weighed more than the list's; the list's rule
now weighs more, and a browser test checks there are neither.

### The footer and the about page

Decided by the agent, as they follow from rules already agreed. The
footer names the licence of the page's own content
([card](../done/2026-11-03_show-licences-on-the-site.md)), so it follows
the curriculum, not the address: `curriculum.html` says "This CV may be
copied, not changed", and the home page, now holding the author's
titles and descriptions, says "Reuse with attribution" as the other
pages do. The about page's link to the curriculum points to
`curriculum.html`.

## Results

- **Two slices, one commit each:** the curriculum on its own page
  (`8f8e077`), then the home page as the list of articles (`09ad2e5`).
  Every check of the pipeline passed on both: formatting, lint, 150
  unit tests, the browser tests' types, 820 browser tests.
- **Validated by the user** on the site served locally: the home page,
  the way from a title to its article and back by the logo, the way
  from the footer to the about page and on to the curriculum, on both
  backgrounds: "all fine".
