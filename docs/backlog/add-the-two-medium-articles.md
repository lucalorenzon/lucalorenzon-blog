# Add the two articles already on Medium

- **Opened:** 2026-11-02, by the user; in Now.
- **Status:** noticed, not started.

## Why

The site is meant to showcase the author's experience through articles,
and today it has none. Two are already published on Medium:

- "After Save Event":
  https://medium.com/@luca.lorenzon/after-save-event-905e7b9c2ff1
- "How generate hybris PK values without hybris":
  https://medium.com/@luca.lorenzon/how-generate-hybris-pk-values-without-hybris-e6b4d6c5ea05

They are the first articles to bring onto the site. The card
[add site navigation](add-site-navigation.md) was planned to come right
after them, when the home page becomes the list of contents.

## Known from the start

- **Sources:** the author's notes, outside this repository:
  `~/Notes/hybris/afterSaveEvent/` (`afterSaveEvent.md`, 280 lines, and
  `img.png`, 645 by 510 pixels) and `~/Notes/hybris/howGeneratePKWithoutHybris/`
  (`howGeneratePKWithoutHybris.md`, 120 lines, and `img.png`, 775 by
  148 pixels). Both images have the same name, so each article needs a
  folder of its own. Whether these notes match the versions published
  on Medium is not known.
- **Where they go, for now:** copied by hand under `content/`, as the
  curriculum was. Taking content from other repositories stays in the
  card [take the site's content from several repositories](take-content-from-several-repositories.md).
- **The Markdown holds HTML:** "After Save Event" links to the other
  article with a bare `<a>` with no address. A unit test forbids HTML in
  the content; it becomes a Markdown link to the other article on the
  site.
- **Code blocks:** "After Save Event" has 14 blocks and the other
  article 9 (Java, XML, properties), some with lines well over 80
  characters. The site has no look for code yet: font, colours, what
  happens to long lines on a phone. The readability test counts
  characters in paragraphs and headings, not in code, but its checks on
  horizontal scroll and cut-off text apply to the whole page.
- **Images:** today their text alternative is "img.png", which says
  nothing (WCAG 1.1.1). The PK article's image shows the bits of a
  primary key: a complex image, which needs a description of what it
  shows.
- **Notes in quotes:** both articles use blockquotes for notes and
  disclaimers; the site has no look for them yet.
- **No page but the home exists:** the build writes `index.html` only.
  Each article needs a page of its own, an address, and a way to be
  reached while the navigation does not exist.
- **The same text on two sites:** search engines may treat the copy as
  duplicate content; a canonical link tells them which address is the
  original.
- **The repository is public:** what the author publishes about past
  work must hold no employer internals; the articles describe the
  platform, but are to be read once more with that in mind.
- **The articles' licence:** decided on 2026-11-02
  ([card](../done/2026-11-02_choose-articles-licence.md)): the text
  and the author's own images under CC BY 4.0, the code samples under
  MIT-0, unless otherwise noted; images by others keep their own terms
  and name their source, so each image is to be checked. Showing it on
  the pages is the card
  [show the licences on the site](show-licences-on-the-site.md),
  before this one.

## Questions

- Which version is the source: the notes or the text on Medium; and
  whether the text is revised (typos, wording) or brought as it is.
- The address of each article, and how it is reached without the
  navigation: a link from the home page, or only the address.
- The look of code blocks, blockquotes and images, within the
  readability rules already in force.
- Which address is canonical, the site's or Medium's.
- How thin the first slice can be: one article first, the shorter one,
  then the other.
