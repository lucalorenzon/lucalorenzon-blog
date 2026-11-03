# State that the views are the author's own

- **Opened:** 2026-11-03, by the user, from the card
  [show the licences on the site](2026-11-03_show-licences-on-the-site.md);
  first in Next, moved to Now by the user the same day, before the card
  [add the two articles already on Medium](../backlog/add-the-two-medium-articles.md).
- **Status:** done on 2026-11-03, published with version 0.8.0.

## Why

The author is employed, and writes about technologies and problems met
at work, past and present; the first two articles describe a platform
the author worked on for past employers. A statement such as "the views
here are my own, not my employer's" tells readers, and employers, that
an article speaks for the author only. It came up while choosing the
name of the licence page: on Simon Willison's and Martin Fowler's
sites, "Disclosures" is where such statements live. The user moved the
card ahead of the articles so the statement is live before they are.

## The answers

### What it says

**Chosen: a generic statement that names no employer:** "The views on
this site are my own. They do not represent those of my employers,
current or past." The curriculum already names no employer ("Italian
Luxury Brand"); naming one here would undo that, and let a reader link
the name to the articles. "Current or past" covers the employers the
first articles are about.

### Where it lives

**Chosen: a section of a new about page, `about.html`**, the user's
proposal, modelled on Eli Bendersky's "About", which holds his
introduction, the same kind of statement and the licence of his code.
The page has an introduction (who the author is, what the site is for,
a link to the curriculum), the statement, and a summary of the reuse
terms with a link to `licence.html`, which stays as it is. The
curriculum stays on the home page; the about page only links to it.

The footer's link ("Reuse with attribution", "This CV may be copied,
not changed") now opens the about page. The reader who follows it
finds the reuse terms in short on that page, and the full terms one
link further; the licence page no longer links to itself.

On 2026-11-02 an "About" page had been rejected because it would
overlap with the curriculum on the home page; with the curriculum only
linked from it, that reason no longer holds.

Rejected:

- **In the footer, as a third item:** seen on every page, but the line
  grows and wraps on phones.
- **A section of the licence page:** cheap, but the user had chosen that
  page for licences only, and a reader following "Reuse with
  attribution" does not expect it there.
- **At the end of each article:** next to what it concerns, but there
  are no articles yet, so it could not go live first.
- **A page of its own, `disclosure.html`:** tidy, but a page and a
  footer link for two sentences; the user chose a section of the about
  page instead.
- **The full licence text inside the about page, with `licence.html`
  removed:** the user kept the licence page and a summary on the about
  page.

### The introduction's wording

The user corrected the agent's draft: "product engineer" rather than
"software engineer and architect"; not only what was learned at work,
but also thoughts and doubts on topics of public debate, and stories of
paths taken, good or bad. The link to the curriculum reads "my
curriculum" rather than "home page", the user's point being that the
curriculum will leave the home page once the navigation comes; the user
first proposed "here", and chose "my curriculum" after the agent
pointed out that "here" says nothing in a list of links (WCAG 2.4.9,
level AAA, which the site aims at though no tool checks it).

### How it is checked

As for the licence page: the same unit and browser checks as the home
page. The about page opens with a paragraph, so its first section has a
line above it; the content layout check now expects that line exactly
when a page does not open with a section.

## Results

Published with version 0.8.0: build run
[41551617802](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41551617802)
and deploy run
[41551621627](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41551621627)
passed, the browser test on the live site included. On
`https://www.lucalorenzon.com/` the footer's link opens `about.html`,
which answers 200. Locally, 58 unit tests and 148 browser tests passed
(one skipped, the live-only check).
