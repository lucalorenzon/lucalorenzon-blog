# Add a sitemap

- **Opened:** 2026-11-06, by the user, while choosing on which pages the
  SEO check runs
  ([card](check-seo-with-lighthouse.md)). In Now, before the SEO
  check is resumed.
- **Status:** being worked on, since 2026-11-06.

## Why

The site has no sitemap: a file, usually `/sitemap.xml`, listing the
pages the site asks search engines to index. The user had meant to open
a card for it, as for the missing `robots.txt`.

It also answers a need of the SEO check: that check runs on the local
build and on the live site, and on the live site there is no folder to
walk, so it needs a list of pages. Today the only list is written by
hand in the tests. The user chose to put the SEO check on hold, add the
sitemap first, and then take the check's list of pages from it, in both
moments.

## Answers so far

### Which fields, and where the date comes from?

Chosen by the user: each entry has the page's address and the date of
its last significant change (`<lastmod>`). The build computes it from
git, before publication: the latest of the last change to the page's
content and the last change to the code that turns that content into
the page's text, links or data. A change of appearance only (the
stylesheet, the fonts, the layout) does not move the date.

- **Why a date at all:** it is the only optional field search engines
  use. Google's documentation
  ([Build and submit a sitemap](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap),
  read on 2026-11-06) says Google ignores `<priority>` and
  `<changefreq>`, and uses `<lastmod>` "if it's consistently and
  verifiably accurate".
- **Why not appearance:** the same page says the date "should reflect
  the date and time of the last significant update to the page": the
  main content, the structured data or the links are significant, the
  copyright date is not. A change of appearance is closer to the
  copyright date; counting it would make the date move when nothing
  significant changed, and risk Google ignoring the field for the site.
- **Why the code too, not the content alone:** part of the code changes
  what the page says: the header's and footer's links, the tags in the
  page's head, how Markdown becomes text.
- **Built before publication:** the published site is always the one
  built and tested from a commit of `main`, through the snapshot or the
  rebuild, so the date is computed there.
- **Git history in the pipeline:** the build job downloads only the
  last commit today, so every file would get the same date and every
  page would look changed at each build. How the dates are read instead:
  "How are the dates read from git?" below.
- **Rejected:** the address alone, always accurate but silent on
  changes; every change to the rendering code, appearance included,
  which Google does not count as significant; the build's own date,
  which changes at every build.

### How is the significant change found, in this first increment?

Chosen by the user: the date of a page is the date of the last commit
that changed its Markdown file. It is knowingly imprecise for now, and
becomes accurate with the card
[move titles, descriptions and dates into the Markdown's front matter](../backlog/move-page-fields-into-markdown-front-matter.md).

- **The problem:** the code that changes what a page says and the code
  that changes only how it looks are in the same files, often on the
  same line: the Tailwind classes are written in the Rust code next to
  the links and texts (9 in `src/page_frame.rs`, which draws every
  page's frame; 35 in `src/markdown.rs`). Git sees which files and lines
  changed, not whether the change is of content or appearance.
- **Why the Markdown:** it already holds each page's body, the pure
  content. The user wants it to become the only source of content, with
  the titles, descriptions and dates now in the code moved into a YAML
  front matter in each file: a card of its own.
- **The imprecision, accepted for now:** a change to an article's title,
  description or date, to a page's description, or to the links of the
  header and footer does not move the date.
- **Rejected:** a list of files per page, where a file holding both
  content and appearance counts entirely, so a change of colour would
  move every page's date; a fingerprint of each generated page's
  significant parts, compared with the previous version's, accurate but
  needing the previous fingerprints and dates, which the build keeps
  nowhere.

### Which date does the home page get?

Chosen by the user: none, for now. The home page is in the sitemap with
its address only; the date is optional entry by entry.

- **The problem:** the home page has no Markdown of its own; its content
  is the list of articles (title, description and date of each), all in
  the code.
- **When the home page changes:** in the user's words, when an article
  is added or removed. Using that date is a card of its own, by the
  user's choice:
  [give the home page a date in the sitemap](../backlog/date-the-home-page-in-the-sitemap.md).
- **Rejected:** the latest date among the articles' Markdown files,
  wrong in the most common case, since correcting an article's text
  would move the home page's date while the home page, which shows only
  title, description and date, does not change; the date of the home
  page's code files, which brings back the mixed files already
  rejected.

### Where does the list of pages come from?

Chosen by the user, revised on 2026-11-07: from the Markdown files under
`content/`, from which the build writes the pages. The date of a page is
the date of the Markdown file the build writes it from; the build knows
it while writing the page, and writes the sitemap's entry with it.
`README.md` and `LICENSE-MIT-0`, in `content/` but not pages, are on an
exclusion list.

- **The model the user aims at,** with the card
  [move titles, descriptions and dates into the Markdown's front matter](../backlog/move-page-fields-into-markdown-front-matter.md):
  the build goes through every `.md` file under `content/` and its
  folders, reads its front matter (title, description, slug) and its
  date of last change, and from them writes both the page and its
  sitemap entry. The address comes from the slug, which follows the
  marketing campaigns, not the file's name. Old slugs, and redirects
  from them, are left out of this card.
- **Pages without a Markdown file:** today only the home page, which
  lists the articles, and the "page not found", which carries `noindex`
  and stays out of the sitemap. Fixed pages without Markdown, should
  there ever be any, are dealt with when they arrive.
- **What changed from the first answer:** on 2026-11-06 the user had
  chosen to read the pages already written in the built site's folder,
  leaving out those that carry `noindex`, so that the sitemap would not
  depend on how a page is produced. That left the link between a page
  and its Markdown file to be rebuilt another way: by a rule on names,
  implicit and fragile; or by a record the build writes while writing
  the pages, which makes the sitemap depend on the build again. Thinking
  it through, the user moved the source of the list to `content/`
  itself: the list is not written by hand, it is the folder.
- **The problem it solves:** the build program (`src/main.rs`) writes
  the pages one by one, each with its own function, which decides inside
  whether the page is indexed; each page's Markdown is named in its own
  code, with `include_str!`. No single place knows "these are the
  indexed pages, at these addresses, with these Markdown files".
- **The exclusion list** belongs to the process that goes through
  `content/`: the step that reads the dates now, and the build itself
  once it goes through every `.md` file.
- **Rejected:** the pages written in the built site's folder (the first
  answer, see above); a list of pages kept by hand beside the calls that
  write them, free to disagree with them, so a new page could be
  written and forgotten in the sitemap; a rule on names (`X.html` from
  `content/X.md`), which an address chosen for a campaign would break.

### How are the dates read from git?

Chosen by the user: by a step of its own, before the build. It reads
the whole history of commits without the files' contents, writes the
date of each Markdown file's last change into a text file, and hands
that file to the build, which checks out the last commit only, as
today, and takes the dates from the file.

- **In the pipeline:** a job of its own clones the repository with the
  full history but only its metadata (`--filter=blob:none`, no
  `--depth`); `git log -1 -- <file>` needs the commits and their lists
  of files, not the files' contents, so the dates are exact. The
  version of `actions/checkout` pinned in the workflows has both inputs
  this needs, `fetch-depth: 0` and `filter` (read in its `action.yml`
  on 2026-11-07).
- **Locally:** the same step is a script working on the repository
  already there, which has its whole history; `scripts/build-site.sh`
  stays the single entry point.
- **Why a job of its own:** the user asked whether the approach holds
  when articles come from several remote repositories
  ([card](../backlog/take-content-from-several-repositories.md)). With
  a single build job, the build would have to know every source, hold
  the credentials of the private ones (the curriculum's repository is
  private) and build the site. With a job of its own, that job becomes
  the step that gathers the content: for each source it reads the dates
  and takes the files, and hands the build the Markdown files with
  their dates. The build does not know where the content comes from,
  does not use git and sees no credentials; a new source changes only
  that job. The same file can later carry the dates of articles added
  or removed, for the
  [home page's date](../backlog/date-the-home-page-in-the-sitemap.md).
- **Not known yet:** how external sources will arrive (a clone, a
  submodule, a package, a release file). A source without git history
  would have to give its dates itself, for example in its front matter;
  the gathering step is where that is decided, source by source.
- **Measured on 2026-11-07,** on this repository: with the last two
  commits only, four of five Markdown files got a wrong date, since git
  gives every file unchanged in them the date of the oldest commit it
  has. With `--depth=100` too: `content/curriculum.md`, last changed
  164 commits earlier, got 2026-11-03 instead of 2026-11-01; most
  commits are documentation, so content files age fast when counted in
  commits. A full clone weighs 3.3 MB.
- **Which date of the commit:** the author's, the day the change was
  written, chosen by the user for now. The committer's date is when the
  commit was last applied, and `git pull --rebase` changes it for
  commits not yet pushed: on 2026-11-07 the two fell on different days
  in 13 of 413 commits, while for the last change of each content
  Markdown file they coincide.
  Neither is the day the page changes on the site, which is the
  publication of the version that carries it: the card
  [use the publication date in the sitemap](../backlog/use-publication-date-in-the-sitemap.md),
  by the user's choice.
- **Rejected:** the last two commits, which would need the previous
  build's dates kept somewhere, which the build does not do, and one
  new commit per build, while a push often brings many; a fixed depth,
  which sooner or later misses a page unchanged for longer; a sparse
  checkout, which would leave out the folders the build needs; a single
  build job with the full history, simpler and the same as locally, but
  it would carry every future source and its credentials.

## Known from the start

- **The build already decides which pages are indexed:** each page is
  `Indexing::Indexed` or `Indexing::NoIndex` (`src/page_frame.rs`); the
  "page not found" is the only `NoIndex` page today, and would not be in
  the sitemap.
- **The build already knows the site's full address,** `SiteAddress`
  (`src/site_address.rs`), used for the canonical link; a sitemap needs
  full addresses too.
- **The format** is the sitemaps.org protocol: an XML file, one entry
  per page with its full address (`<loc>`), and optional fields such as
  the date of the last change (`<lastmod>`).
- **A page missing from the sitemap** would escape both the search
  engines and the SEO check. Locally a test can compare the sitemap with
  the pages found in the built site's folder.
- **The site is served from the root** of `https://www.lucalorenzon.com/`
  since 2026-11-01, where a sitemap is expected.
- **Pointing search engines to it** is the job of a `robots.txt`
  ([card](../backlog/add-robots-txt.md)).
- **The next version waits for this card,** the `robots.txt` and the SEO
  check: the user chose to publish them together with the canonical
  link, the descriptions and the Open Graph tags already done.

## Questions

- What the build does when the file of dates is missing or lacks a
  page.
- How the test checks it, locally and on the live site.
