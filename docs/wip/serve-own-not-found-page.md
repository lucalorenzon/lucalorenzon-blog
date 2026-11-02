# Serve the site's own "page not found"

- **Opened:** 2026-11-01
- **Status:** in progress since 2026-11-02, in Now.

## Why

An address that does not exist on the site, for example
`https://www.lucalorenzon.com/missing-page`, gets GitHub's own page,
titled "Page not found · GitHub Pages", which shows GitHub's text and
branding. The user wants a page that gives no information at all, just
"404".

GitHub Pages serves a file named `404.html` at the root of the published
site, with status 404, for every address that does not exist; this works
with a deployment from GitHub Actions too. The build writes only
`index.html` today (`src/main.rs`), so there is none.

## Limits known from the start

- "No information" applies to the page's content, not to the whole
  answer: GitHub Pages adds the header `server: GitHub.com`, which cannot
  be removed, and the site's numeric addresses are public GitHub ones.
- It does not cover the page "There isn't a GitHub Pages site here",
  which GitHub serves for a domain it does not recognise, outside the
  site (seen on 2026-11-01 as a stale cache entry right after the custom
  domain was set, card `choose-site-address`).

## Questions to answer

- **What the page shows.** The user's proposal: only "404". Whether it
  also carries the site's name, a link to the home page, the site's
  look, and in which language.
- **How it reaches its stylesheet and favicon, if it uses them.** The page
  is served at any depth (`/a/b/c` too), so the relative paths the home
  page uses (`style.css`, `favicon.ico`) would break there. A unit test
  in `src/home_page.rs` forbids paths starting from the root
  (`href="/`), a rule from when the site lived under a sub-path; since
  2026-11-01 it lives at the root of `www.lucalorenzon.com`. A page with
  no stylesheet avoids the question.
- **How it is checked.** Presumably in the browser test: a missing
  address answers 404 with the page's content, and the page passes the
  accessibility checks like the home page.
