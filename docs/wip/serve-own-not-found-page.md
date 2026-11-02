# Serve the site's own "page not found"

- **Opened:** 2026-11-01
- **Status:** in progress since 2026-11-02; parked the same day for
  the card on how pages write the addresses of their files, and resumed
  once it closed.

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

## Answers so far

Given by the user on 2026-11-02:

- **The proposal of "only 404" is dropped.** It came from a memory of
  an application answering an AJAX call, where a short answer avoids
  handing information to attackers; it does not apply to a static page
  on GitHub Pages. The page gives the visitor the minimum to find their
  way.
- **Tab title:** "Page not found - Luca Lorenzon". Every page follows
  the same order, the page's name first and the site's name after, so
  the page is told apart where only the start of the title shows (many
  open tabs, bookmarks); the home page becomes "Curriculum Vitae - Luca
  Lorenzon". The earlier order had no reason of its own: the rule of
  2026-10-28 only asked that the author's name be there
  ([card](../done/2026-10-28_add-header-footer-and-favicon.md)).
- **Header and footer:** the same as the home page, with "Page not
  found" as the main heading; only the content differs.
- **Content:** a link to the home page, "Go to the home page".
- **Stylesheet and icon:** answered by the card
  [choose how pages write the addresses of their files](../done/2026-11-02_choose-how-pages-write-addresses.md)
  ([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md)).
  The build takes the site's full address, by default
  `https://www.lucalorenzon.com/`; this page writes its addresses from
  its path part (`/` today), while every other page keeps relative
  paths. This card introduces that setting.
- **Check:** locally, the browser test opens the page directly; the
  local server (Python's `http.server`) answers a missing address with
  its own page, not with `404.html`. The real behaviour, status 404 at
  any address, is seen on the live site.
- **Search engines:** the page goes on the block-list of the card
  [add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md).

## Results so far

Implemented on 2026-11-02 in three commits: the home page's tab title
(`f29338d`), a page frame shared by every page (`f58071f`, the home
page's HTML unchanged byte for byte), and the setting with `404.html`
(`1da48fc`). The build checks the site's address and stops on an
invalid one; unit tests cover both pages and the address; the browser
test opens `404.html` locally, follows its link home and runs the
accessibility checks on it. The check of a missing deep address on the
live site (status 404, the page with its look) runs only after the next
publication: the card closes once it passes.
