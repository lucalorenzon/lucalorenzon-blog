# Add the two articles already on Medium

- **Opened:** 2026-11-02, by the user; in Now.
- **Status:** in progress since 2026-11-03.

## Why

The site is meant to showcase the author's experience through articles,
and today it has none. Two are already published on Medium:

- "After Save Event":
  https://medium.com/@luca.lorenzon/after-save-event-905e7b9c2ff1
- "How generate hybris PK values without hybris":
  https://medium.com/@luca.lorenzon/how-generate-hybris-pk-values-without-hybris-e6b4d6c5ea05

They are the first articles to bring onto the site. The card
[add site navigation](../backlog/add-site-navigation.md) was planned to come right
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
  card [take the site's content from several repositories](../backlog/take-content-from-several-repositories.md).
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
  and name their source, so each image is to be checked. Shown on the
  site since version 0.7.0
  ([card](../done/2026-11-03_show-licences-on-the-site.md)): an
  article's page gets the footer "Reuse with attribution", which links
  to `about.html`, and from there to `licence.html`.

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

## Answers so far

Gathered on 2026-11-03, article by article.

### Both articles

- **The source is the text published on Medium, not the notes.** The
  notes turned out to hold more than was published: the author did not
  remember it, the articles being three years old. Everything said
  above about the notes (line counts, number of code blocks, the bare
  `<a>`) is therefore to be checked again on the published text.
- **Explaining the platform's behaviour is the point of the articles.**
  What belongs to the vendor is the form of its code and of its
  drawings, not the behaviour of the product. Configuration keys, their
  default values, the fields of a class and the interfaces a developer
  implements are usage information and can be quoted. Method bodies
  taken from the decompiled platform would not be: the published
  "After Save Event" has none.

### "After Save Event"

The author pasted the Medium text into the session; the formatting
(headings, code blocks, lists, links, images) did not survive the paste
and is to be rebuilt from the notes, which share the structure.

- **Code:** what remains is the configuration in `advanced.properties`,
  the fields of `AfterSaveEvent`, the `AfterSaveListener` interface, the
  author's snippets on the type code, three lines of `core-spring.xml`
  and an empty outline of `notifyCommit`. All of it is usage
  information or the author's own.
- **The comments in the configuration** are most likely the author's:
  their typos are the author's, copied from the notes. Still open: the
  author may check them on an old backup; if that is not possible, they
  are reworded.
- **The transaction diagram** comes from the vendor's documentation and
  is on Medium. It is replaced by a table written by the author
  (sequence of operations → event generated). Chosen because the image,
  being complex, needed a text description anyway, and that description
  is the table: one element instead of two, all of it the author's and
  under CC BY 4.0. Rejected: keeping the vendor's diagram with its
  source and a description, which keeps the look of the Medium version
  but requires checking the vendor's terms of use.
- **Still open:** whether typos and slips are corrected, for example
  `core.aftersave.intervals`, twice in the text, where the key listed
  above it is `core.aftersave.interval`.

### "How generate hybris PK values without hybris"

Not looked at yet on the published text.
