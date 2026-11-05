# Make the browser tests check behaviour, not articles

- **Opened:** 2026-11-05, by the user, reviewing the browser test of
  the rule "a table cell with no space never wraps", in the card
  [let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md);
  in Next.
- **Status:** noticed, not started.

## Why

The first version of that test looked for the PK bits table of the
hybris article and for its values ("63–44", "43", "42–15", "14–0").
The user found it wired to one article rather than checking that the
behaviour is right: if the article changed, the test would break with
the rule still working, and it would say nothing about a table in
another article. It was rewritten before its commit as a general rule
over the site's pages; the same weakness is in the tests already
there.

The unit tests in Rust do not have it: they check the conversion on
small invented Markdown. The browser tests do:

- `end2end/tests/support.ts` lists the pages the content and
  readability tests check (`TEXT_PAGES`), each with flags about its
  content (`hasRoles`, `hasConsecutiveParagraphs`), and a comment on
  which article stays out and why.
- `code-blocks.spec.ts` lists the two articles (`ARTICLES`) and skips
  one check for the hybris article by name.
- `accessibility.spec.ts` has its own list of pages (`PAGES`) and of
  expected accessibility trees per page (`TREE_PAGES`).

Each new article means editing these lists, and the tests know facts
about the content that belong to the content.

## Proposal

Open, to be decided when the card is picked up. Two roads, not
exclusive:

- **Test pages built only for the tests,** with content made to
  exercise each rule (a table with single values, a wide table, wide
  code, consecutive paragraphs), kept out of the published site. The
  tests then know their fixtures, not the articles.
- **General properties over every page,** found without naming them
  (for example from a list the build writes, or by following the
  site's links once the navigation exists): every page has no
  sideways scroll, every single-value cell is on one line, and so on.

## Known from the start

- **A general property can pass by checking nothing:** if no page had
  a table, the table test would pass with no cell checked. Test pages
  built for the tests would close that gap; a general property alone
  needs a guard.
- **Pages left out on purpose** (today the hybris article, until its
  wide table is handled) need a way to be declared that does not live
  inside each test.
- **The live check:** the same suite runs on the published site after
  each publication (`BASE_URL`); test pages kept out of the site would
  not exist there.
