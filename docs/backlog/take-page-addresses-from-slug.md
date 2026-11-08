# Take the pages' addresses from a slug, with redirects

- **Opened:** 2026-11-08, by the user, splitting the card on moving
  titles, descriptions and dates into the Markdown's front matter,
  opened on 2026-11-06 while choosing where the sitemap's dates come
  from ([card](../done/2026-11-07_add-sitemap.md)). The articles' other
  fields come first ([card](../wip/read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

Today each article's address is written in the code, in `ARTICLES` of
`src/article_page.rs`, next to the name of its Markdown file. The model
the user aims at (2026-11-07, while making the
[sitemap](../done/2026-11-07_add-sitemap.md)): the build goes through
every `.md` file under `content/` and its folders, reads its front
matter and its date of last change, and from them writes both the page
and its sitemap entry. The page's address comes from a slug in the
front matter, which follows the marketing campaigns and may differ from
the file's name.

The user kept it out of the articles' first slice: changing an address
brings questions of its own.

## Known from the start

- **Files in `content/` that are not pages,** `README.md` and
  `LICENSE-MIT-0`, are on an exclusion list, chosen by the user for the
  sitemap's card.
- **The site is static, on GitHub Pages:** there is no server to answer
  an old address with a redirect.

## Questions

- What the build does when two pages have the same slug.
- How an old address keeps working when a slug changes: a list of old
  slugs in the front matter, and what each old address serves on a
  static site.
- Whether the build finds the pages by going through `content/`, in
  this card or in one of its own.
