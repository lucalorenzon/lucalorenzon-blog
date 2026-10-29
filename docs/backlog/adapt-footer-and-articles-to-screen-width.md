# Adapt the footer and the articles to screen width

- **Opened:** 2026-10-29, split from the card "Make the site responsive"
  (opened 2026-10-28). Its other half is
  [adapt the header to screen width](../done/2026-10-29_adapt-header-to-screen-width.md).
- **Status:** noticed, not started.

## Why

Once the header adapts to screen width, the rest of the page has to
follow. The footer is a single line of text today and probably needs
nothing, but nobody has looked at it on a phone. Articles do not exist
yet; adapting them now would mean designing something that is not
there.

## Decisions already taken

Agreed with the user on 2026-10-29, while the header card was being
discussed.

- **Titles are at most 50 characters.** A first figure of 80 was
  replaced after seeing long titles in the playground. Medium states no
  official limit; the figures found were about 100 characters for title
  and subtitle together in previews, about 60 to show in full in search
  results, under 95 as one author's rule.
- **Beside the logo a title takes at most two lines.** With 50
  characters this holds from 1280px, where the title moves beside the
  logo ([card](../done/2026-10-29_adapt-header-to-screen-width.md)); below that the
  title sits under the header line and wraps freely. So no title needs to
  be measured one by one, as first thought: the limit and the threshold
  are enough. The browser test of the header already checks two
  50-character titles.

## Questions

- How the 50-character limit is enforced when articles are written.
- Whether the footer needs anything on a phone.
- How an article's body adapts: text width, code blocks, images.
