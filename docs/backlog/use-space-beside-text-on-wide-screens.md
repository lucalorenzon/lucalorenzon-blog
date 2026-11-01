# Use the space beside the text on wide screens

- **Opened:** 2026-11-02, by the user, reading version 0.5.0 on the
  live site, at the end of the card
  [improve the site's readability](../done/2026-11-02_improve-site-readability.md).
- **Status:** noticed, not started.

## Why

Version 0.5.0 made the text much more readable, in part by narrowing
the text column to 60ch so that no line exceeds 80 characters (WCAG
1.4.8, level AAA). On wide screens, such as 1920 pixels, that single
column leaves much empty space at the sides. The user finds the text
size right: what weighs is the empty space, not small text.

## Known from the start

- **The limit is on the line, not on the screen.** 80 characters is
  how far the eye travels before going back; no single column of text,
  however tuned, fills a wide screen without breaking it. Filling the
  space means changing something else: more columns, other content
  beside the text, or a different arrangement of the page's parts.
- **Larger text was ruled out** by the user: the size is right.
- **The page is the curriculum today,** with sections of different
  kinds: summary, experience, competencies, education, training,
  languages.
- **The home page will change:** the card
  [add site navigation](add-site-navigation.md) expects it to become
  the list of contents once articles arrive, and the navigation itself
  will need room. A layout for wide screens should not have to be
  redone then.
- **A related card exists:**
  [adapt the footer and the articles to screen width](adapt-footer-and-articles-to-screen-width.md),
  in Later, about the footer on phones and how an article's body adapts.
- **The readability test** (`end2end/tests/readability.spec.ts`) checks
  line length, spacing and the reader's spacings on twelve screen sizes;
  any new layout is held to it.

## Questions

- What fills the space: more columns of text, content beside the text
  (for example contacts or competencies next to the experience), the
  future navigation, or something else.
- Whether this card joins the card on adapting the footer and the
  articles to screen width, or stays separate.
- From which width the layout changes, and what stays as it is on
  phones and tablets.
