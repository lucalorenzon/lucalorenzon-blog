# Find the articles in the content folder

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](../wip/read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

With the articles' card, an article's title, description and date come
from its Markdown file, but the list of articles is still written in
the code: `ARTICLE_FILES` in `src/article_page.rs` names each file, its
address, and reads it with `include_str!`. A new article means editing
the code.

The user wants the build to find the articles by reading
`content/articles/` when it runs, instead of a list wired into the
code. It is a step towards the model the user aims at (2026-11-07,
while making the [sitemap](../done/2026-11-07_add-sitemap.md)): the
build goes through every `.md` file under `content/`, reads its front
matter and its date of last change, and writes both the page and its
sitemap entry.

## Known from the start

- **Kept apart from the slug,** by the user: finding the files and
  taking the address from a slug are two decisions
  ([card](take-page-addresses-from-slug.md)). Until that card, the
  address has to come from somewhere else, for example the file's name.
- **The text is read when the build runs, not when it is compiled:**
  today `include_str!` puts it in the program, so a missing file is a
  compile error; read from the folder, it becomes an error of the
  build, which needs a rule of its own, and the build needs to know
  where `content/` is.
- **Files in `content/` that are not pages,** `README.md` and
  `LICENSE-MIT-0`, are on an exclusion list, written twice today: in
  `src/sitemap.rs`'s tests and in `scripts/content-dates/content_dates.py`.
- **The articles' rule for a missing or wrong field** applies to every
  file found ([card](../wip/read-article-fields-from-front-matter.md)).

## Questions

- Where the address comes from until the slug's card: the file's name,
  or something else.
- What the build does with a file in `content/articles/` that is not an
  article, or a folder with no articles at all.
- Whether only the articles are found this way, or the other pages
  too.
