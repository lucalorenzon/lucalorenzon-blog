# Adapt the footer and the articles to screen width

- **Opened:** 2026-10-29, split from the card "Make the site responsive"
  (opened 2026-10-28). Its other half is
  [adapt the header to screen width](adapt-header-to-screen-width.md).
- **Status:** noticed, not started.

## Why

Once the header adapts to screen width, the rest of the page has to
follow. The footer is a single line of text today and probably needs
nothing, but nobody has looked at it on a phone. Articles do not exist
yet; adapting them now would mean designing something that is not
there.

## Decisions already taken

Agreed with the user on 2026-10-29, while the header card was being
discussed; they apply as soon as page titles can be long, that is with
the first article.

- **On wide screens the title in the header may take two lines.** With
  the ASCII-art logo on both sides, the centre column holds about 22
  characters per line at 1024px, 38 at 1280px, 54 at 1536px; less in
  practice, because words are not broken.
- **A title that does not fit in two lines goes into the body of the
  page**, as it does on small screens. Style alone cannot count lines,
  but the font has a fixed width, so the build can work out for each
  title from which width it fits in the header. No JavaScript needed.
- **Titles have a maximum length**, still to be chosen. Medium states no
  official limit; the figures found are about 100 characters for title
  and subtitle together in previews, about 60 to show in full in search
  results, under 95 as one author's rule. The length chosen decides how
  often the rule above applies: with 44 characters a title always fits
  in two lines from 1024px.

## Questions

- What the maximum title length is.
- Whether the footer needs anything on a phone.
- How an article's body adapts: text width, code blocks, images.
