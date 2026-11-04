# Give the articles' code, quotes and tables a look

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Next. Narrowed on 2026-11-04,
  when what wide code and tables do on a phone moved to the card
  [let the Markdown ask for components for wide code and tables](annotate-markdown-for-wide-content.md).
- **Status:** noticed, not started.

## Why

The articles from Medium bring three kinds of content the site has no
look for: code blocks, notes in quotes and tables. Until this card is
done, the [article pages](../done/2026-11-04_add-article-pages.md) show them with the
default style. The user wants the article pages accessible and in as
good a state as the curriculum before the navigation links to them, so
this card comes before the [navigation](add-site-navigation.md).

## Known from the start

- **Code blocks:** Java, XML, Groovy, SQL and properties. Undecided:
  font and colours. What long lines do on a phone belongs to the card
  on wide content.
- **Quotes:** both articles open a note or a warning with a blockquote.
- **Tables:** "After Save Event" has one (the transaction cases), the
  article on hybris PK values two (the bits of a key and the
  `numberseries` table). Their look is here; what a table too wide for
  the screen does belongs to the card on wide content.
- **Rules already in force:** the readability rules and their test
  ([card](../done/2026-11-02_improve-site-readability.md)), and the
  contrast the WCAG criteria ask for.
- **"Done" means** the automatic checks pass on the article pages, as
  they do on the curriculum, and the user validates the pages in the
  browser.

## From the user's first reading of the article pages

On 2026-11-04 the user read the two article pages, built with the
default style, before closing the card that gave them a page
([card](../done/2026-11-04_add-article-pages.md)), and asked for the
following; the card was closed and these moved here.

- **The text is too small,** Literata included, "on all the text". On
  2026-11-02 the user had judged the size right
  ([card](../done/2026-11-02_improve-site-readability.md), results);
  long technical articles changed that reading. Open: whether the size
  changes for the articles only or for the curriculum too.
- **Code in a box with a visible border,** as on Medium: a styled
  block, not a form field. The minimum for code. Today code uses the
  site's monospace font, IBM VGA 8x16, the logo's pixel font, which no
  one chose for code and reads smaller than Literata at the same size.
- **Headings of articles, in the spirit of an 1980s BBS text:** section
  titles (`h2`) in inverse video like the curriculum's roles, and
  larger; sub-titles (`h3`) bold only; both set apart by blank lines.
  Today an article's `h3` takes the style of the curriculum's roles
  and weighs more than the section titles.
- **No line above the sections:** the amber line is out of place in an
  article; blank lines and bold titles instead.
- **For the articles only:** a curriculum is not an article, the
  things to highlight differ. The curriculum keeps its headings and
  its section lines.

## Questions

- The look of each kind, within the readability rules.
- The text size, and whether it changes for the curriculum too.
- How the article styles are kept apart from the curriculum's, which
  share the `long-text-column` rules today.
