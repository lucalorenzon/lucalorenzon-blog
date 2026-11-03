# Bring the article on generating hybris PK values into the site's Markdown

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Now.
- **Status:** done on 2026-11-03. The file is
  `content/articles/how-to-generate-hybris-pk-values-without-hybris.md`;
  not shown on the site until the card
  [give each article a page of its own](../backlog/add-article-pages.md).

## Why

Same reason and same split as the card
[bring "After Save Event" into the site's Markdown](2026-11-03_add-after-save-event-markdown.md),
done the same day. This card wrote the text of "How generate hybris PK
values without hybris"
(https://medium.com/@luca.lorenzon/how-generate-hybris-pk-values-without-hybris-e6b4d6c5ea05)
as Markdown, checked by the unit test that forbids HTML in the content
until the pages make it visible.

## The answers

Most answers were given for both articles while working on the first
one, and are explained in [its card](2026-11-03_add-after-save-event-markdown.md):
the Medium text is the source, not the author's notes
(`~/Notes/hybris/howGeneratePKWithoutHybris/howGeneratePKWithoutHybris.md`);
explaining the platform's behaviour is the point, while the form of the
vendor's code and drawings belongs to the vendor; typos and slips are
corrected; the file lives in `content/articles/`. What is specific to
this article:

### What is quoted from the platform

Little: `getCacheSize`, introduced on Medium with "it does something
similar to this" and reduced to three comments describing its logic,
and the signature of `resetSeries`. The Groovy that reads the type
code, the call that resets the counter, the query on the counter and
`calculatePK` are the author's. **Chosen, by the user: the same closing
sentence as the other article**, saying that what is quoted from SAP
Commerce belongs to SAP and MIT-0 covers only the author's code, since
the footer speaks of all the code on the page.

### The bit diagram

**Chosen, by the user: a table written by the author** (bits 63–44,
43, 42–15, 14–0, with what each holds), for the same reason as the
transaction diagram of the other article: the image comes from the
vendor's documentation and needed a text description anyway. Rejected
for the same reason: keeping the vendor's image.

### The `numberseries` table

The author's description of a table in the platform's database. Drawn
by hand on Medium, with the descriptions broken over several lines and
full stops left inside them; it is now a real Markdown table. The
"Extra" column, empty in every row, was dropped, since tables are the
widest content on a phone.

### The corrections

- The title: "How to generate hybris PK values without hybris"; the
  file is named after it.
- **The missing step is restored:** the text used `counterValue` when
  resetting the counter without saying where it comes from. The query
  that reads it comes back from the notes, with one new sentence saying
  which counter values the reset reserves.
- **The two links at the end** now say where they lead: the title of
  the answer on SAP's community site, and "SAP's official
  documentation, version 2105" instead of "here". Their addresses come
  from the notes, as the paste from Medium lost them. Checked on
  2026-11-03: the documentation answers, redirected to the portal's
  new address; the community site refuses a program (403), so it was
  left to the author to open it in a browser.
- Smaller ones: "always or 0 or 1", "Also know as", "can be find", a
  space lost after `DefaultCachingSerialNumberGenerator`.

## Results

- The file is added to the unit test that forbids HTML in the content
  (`src/markdown.rs`): 58 tests pass, formatting and lint checks pass.
- Both articles are now in `content/articles/`; making them visible is
  the [pages card](../backlog/add-article-pages.md).
