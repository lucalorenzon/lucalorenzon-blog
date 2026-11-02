# Show the licences on the site

- **Opened:** 2026-11-02, by the user, from the card
  [choose the licence of the articles](../done/2026-11-02_choose-articles-licence.md).
- **Status:** being worked on since 2026-11-02. Before the card
  [add the two articles already on Medium](../backlog/add-the-two-medium-articles.md),
  so the first articles arrive with their licence in sight.

## Why

The licences of the site's text are declared in the repository
(`content/README.md`), but a reader of the site sees only the pages,
where today the footer says "© Luca Lorenzon 2026" and nothing more.
With no licence in sight, a reader can only assume all rights are
reserved, which is the opposite of what the author wants: the articles
are meant to be reused, translated and adapted, with a citation.
Creative Commons asks for the licence to be stated next to the work, so
whoever copies from a page knows on what terms.

## Known from the start

- **One sentence for the whole site**, the same on every page: the text
  is under CC BY 4.0 and the code samples under MIT-0, unless otherwise
  noted. Personal blogs that state a licence do it this way (Eli
  Bendersky, Chris Wellons), rather than page by page.
- **The curriculum is the "otherwise noted":** it is under CC BY-ND 4.0,
  and says so next to it. Today the curriculum is the whole home page.
- **The footer is checked by tests** (`src/home_page.rs`,
  `src/not_found_page.rs`), which look for the copyright line.
- **A change to the published site**, so it brings a new version and an
  entry in `CHANGELOG.md`.

## Questions

- Where the sentence goes: in the footer beside the copyright line, or
  at the end of the content.
- Its wording, and whether each licence name links to its official
  page.
- Where and how the curriculum states its exception.
- How the build checks that the sentence and the exception are there.
