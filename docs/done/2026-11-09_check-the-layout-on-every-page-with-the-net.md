# Check the layout on every page with the net

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  third of its three parts, after
  [take the net's pages from the sitemap](2026-11-09_take-the-nets-pages-from-the-sitemap.md)
  and
  [check accessibility on every page with the net](2026-11-09_check-accessibility-on-every-page-with-the-net.md).
- **Status:** done, 2026-11-09.

## Why

`readability.spec.ts` and `content-layout.spec.ts` ran on `TEXT_PAGES`,
six pages listed by hand, at the five widths of the text column, and
the list carried facts about the content: `hasRoles` (the page has role
headings, level 3) and `hasConsecutiveParagraphs`. A few of their
checks depend on the text itself and belong to the net; the rest are
component checks.

## Answers

### Which checks depend on the text, and so run on every page?

**Sideways scroll at the narrowest width, 320 pixels** (user, on the
agent's correction). The text breaks only at spaces: a word wider than
the column scrolls the page whatever the rest of the style does. A
style rule breaking long words would make it a component check:
[break long words in the text](../backlog/break-long-words-in-the-text.md).

**Sideways scroll with the reader's spacing, at 320 pixels, too**
(user). With the letters spaced as a reader may set them (WCAG 1.4.12)
the same word is about a quarter wider, so a long word overflows there
first. ADR-004's census had made it a component check, accepting that
loss; with the criterion above it no longer held, and the ADR carries
a note. Both measurements are one test per page.

**At most 80 characters per line, at 1280 pixels,** where the column is
already at its widest (704 pixels, text at 20). It is counted on the
lines as the browser breaks them, and with a proportional font no style
rule can bound it.

**Rejected:** sideways scroll as a component check (the agent's first
proposal, which wrongly assumed the text already breaks long words);
bounding it by 80 characters of the widest glyph, which never fit the
column: 80 "W" are 1568 pixels with text at 18 pixels and 1742 at 20,
against a column of 304 to 704 pixels.

### Which content facts are dropped?

**`hasRoles`, `hasConsecutiveParagraphs`, and the rule "the page has at
least one level-2 heading"** (user for the first two; the third by the
same reasoning, agent's proposal). Each asserted a fact about a page's
text, so a change of text broke the test although the site still
worked. They kept their checks from passing with nothing to check; the
worst case's guard, below, does that now.

### Where do the component checks run until the components' test pages exist?

**Once, on the worst case: the After Save Event article, at the five
widths of the text column** (agent's proposal, user's decision). These
checks depend on the column's width, so all five widths stay: line
height, space between paragraphs, no justified text, headings larger
than the text and their colour, no overlap, the gap above the footer,
and, with the reader's spacing, nothing overlapping or cut off.
Measured on 2026-11-09, the article has every kind of text element
these checks look at: consecutive paragraphs, level-2 and level-3
headings, bulleted and numbered lists, code blocks, a table. A guard
fails, naming what is missing, if it stops having them.

**Accepted:** level-4 headings, found only in the curriculum, are not
covered; they enter only the overlap check.

**Rejected:** the curriculum as a second worst case, for its level-4
headings (the agent's question; the user kept one page for now).

## Results

- **Tests:** 300 became 227, and their summed work 170.4 seconds
  became 142.8 (locally, 14 workers: 41.8 and 40.5 seconds of wall
  time):

  | File | Tests | Work |
  |---|---|---|
  | `readability.spec.ts` | 60 → 16 | 13.6 → 4.1 s |
  | `content-layout.spec.ts` | 55 → 26 | 12.3 → 7.4 s |

  Since ADR-004 was written, 913 tests became 227.
- **Faults made on purpose,** in the built site, all caught:
  - a word of 70 characters in the about page: the net's scroll check
    fails at 320;
  - a word of 26 letters in the same page: only the check with the
    reader's spacing fails; with 24 letters it passes. So at 320 pixels
    a word of 26 letters without a space scrolls the page for a reader
    who raises the spacing;
  - the numbered list taken out of the article: the guard fails,
    saying it is no longer the worst case;
  - a justified paragraph in the article: the component check fails.

Commit: `d767f91`.

## Open points

- **The user is not fully convinced** that components should be
  checked on a real article taken as the worst case: their instinct is
  that components are tested on texts written for the purpose. Not yet
  argued; kept for when it is. It meets ADR-004's components' test
  pages, the next slice, where the components' cases are written for
  the build only.
