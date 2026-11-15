# Link the style guide's pages to its index

- **Opened:** 2026-11-14, by the user, while looking at the code block
  page of the card
  [show the code block in the style guide](2026-11-15_show-the-code-block-in-the-style-guide.md);
  in Now, part of [build a design system](build-a-design-system.md).
- **Status:** done, 2026-11-14. Not published yet: the style guide goes
  live when it is complete.

## Why

From the colours page or the code block page, a reader can go back
only to the home page, through the logo. The style guide is read as a
whole, one page per component, so each page needs a way back to its
index, where the other pages are listed.

## Known from the start

- **For the style guide only** (decided by the user on 2026-11-14):
  the link belongs to the style guide's pages, not to a navigation of
  the whole site. The site's navigation is the menu's job
  ([card](../backlog/add-site-menu.md)), later.
- **The first link of its kind:** today the site's only navigation is
  the logo, to the home page, and the footer, to the about and licence
  pages, the same on every page. No page links to the page that
  contains it; the articles do not link to their list either.
- **Every page of the style guide gets it,** the colours and the code
  block today, the table, typography and breakpoint pages when they
  come; the index does not link to itself.

## Answers

- Where the link goes on the page, and what it says. Answered by the
  user on 2026-11-14, from the agent's suggestion: **a breadcrumb at
  the top**, below the header and before the text, in the form
  "Style guide › Code blocks", where "Style guide" leads to the index
  and the current page is not a link; announced to screen readers as a
  navigation named "Breadcrumb", with the current page marked as such.
  It starts from "Style guide", not from "Home": the logo already leads
  to the home page from every page. And **at the bottom, a link back to
  the top of the page**, since the pages are long on a phone (the code
  block page is about seven screens at 320 pixels). Rejected: the link
  at the bottom only, the user's first idea, which a reader arriving
  on the wrong page finds only after scrolling the whole page; and the
  same breadcrumb at the top and the bottom, two identical navigations
  that a screen reader announces twice.
- Whether the way back to the top is for the style guide only. Answered
  by the user on 2026-11-14: **for the style guide only, for now.** The
  articles, the longest pages of the site, get it with a breadcrumb of
  their own in a card of their own
  ([card](../backlog/add-breadcrumbs-and-back-to-top-to-articles.md)).
- What the link at the bottom says. Answered by the user on 2026-11-14:
  **"Back to top"**, the phrase readers find on most sites. Rejected:
  "Up", which in navigation traditionally means the level above (GNU
  Info's manuals, HTML's old `rel="up"`), so a reader could expect it
  to lead to the index, like the breadcrumb; and a screen reader user
  listing the page's links would hear "Up" with no context, while a
  link's text should say where it leads.

## Results

Commit `c8b9c91`.

- **One component for every page of the guide:** `GuideMain`
  (`src/style_guide/navigation.rs`) writes the page's `<main>`: the
  breadcrumb, the page's text, then "Back to top". The colours and code
  block pages use it, and the pages to come get both by using it too.
- **"Back to top" leads to `#top`,** an id on the frame's outer
  element, so it is on every page of the site, with no visible effect.
  Following it brings the keyboard's starting point back to the top as
  well: the next Tab reaches the logo, the first focusable element of
  the page (checked in Chromium by the browser test). The address bar
  shows `#top` afterwards.
- **The breadcrumb's look:** a line one step smaller than the text, in
  the ink, the link underlined like the footer's. The text column's
  rules for lists are outweighed by repeating the selector, as the
  colour samples' boxes already do, not by `!important`: the agent
  asked the sub-agent to align with that precedent before the commit;
  the screenshots before and after were identical.
- **Tests:** two browser tests at 320 and 1280, on the code block page
  only, following ADR-004 (the breadcrumb leads to the index and marks
  the current page; "Back to top" brings back the view and the
  keyboard); 259 browser tests became 263. Six unit tests; 312 became
  318.
