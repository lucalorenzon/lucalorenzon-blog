# Bring the article on generating hybris PK values into the site's Markdown

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Now.
- **Status:** noticed, not started.

## Why

Same reason and same split as the card
[bring "After Save Event" into the site's Markdown](../wip/add-after-save-event-markdown.md).
This card writes the text of "How generate hybris PK values without
hybris"
(https://medium.com/@luca.lorenzon/how-generate-hybris-pk-values-without-hybris-e6b4d6c5ea05)
as a Markdown file under `content/`, checked by the unit test that
forbids HTML in the content until the
[pages card](add-article-pages.md) makes it visible.

## Known from the start

- **The source is the text on Medium, not the author's notes**
  (`~/Notes/hybris/howGeneratePKWithoutHybris/howGeneratePKWithoutHybris.md`),
  which hold more than was published. Pasted by the author on
  2026-11-03, with the formatting lost; rebuilt from the notes.
- **What the code quotes.** `getCacheSize` is introduced with "it does
  something similar to this" and reduced to three comments describing
  its logic; of `resetSeries` only the signature is left. The Groovy
  that reads the type code, the call that resets the counter and
  `calculatePK` are the author's.
- **The `numberseries` table** is the author's description of a table
  in the platform's database. On Medium it is drawn by hand, with the
  descriptions broken over several lines; it becomes a real Markdown
  table.
- **The bit diagram becomes a table** written by the author, for the
  same reason as the transaction diagram of the other article: the
  image comes from the vendor's documentation and needed a text
  description anyway. Rows: bits 63–44, counter, high part; bit 43,
  marker, 1 when the key is counter based, 0 when UUID based; bits
  42–15, counter, low part; bits 14–0, type code.
- **Typos and slips are corrected,** decided by the user. Among them:
  the title ("How Generate" without "to"); full stops left inside the
  table's sentences by the line breaks; "always or 0 or 1"; "Also
  know as"; "can be find"; a space lost after
  `DefaultCachingSerialNumberGenerator`.
- **The missing step is restored:** the text uses `counterValue` when
  it resets the counter, without saying where it comes from. The notes
  read it with the author's query
  `select currentValue from numberseries where serieskey='pk_1055';`,
  which comes back.
- **The two links at the end** (an answer on SAP's community site and
  the official documentation) are rewritten so their text says where
  they lead, not "here"; both addresses are three years old and are
  checked before they are kept.
- **The note in a quote** keeps the Markdown blockquote; its look
  belongs to the [look card](design-look-of-articles.md).
- **The repository is public:** the text is read once more for employer
  internals before it is committed.

## Questions

- Where the file lives, as for the other article.
- The licence of the code quoted from the platform, as for the other
  article: here only a signature and a described outline.
