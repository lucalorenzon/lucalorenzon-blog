# Add a robots.txt

- **Opened:** 2026-11-06, by the user, while choosing on which pages the
  SEO check runs
  ([card](../wip/check-seo-with-lighthouse.md)), where it was the last
  open question. In Now, after the [sitemap](../done/2026-11-07_add-sitemap.md).
- **Status:** being worked on, since 2026-11-07.

## Why

The site has no `robots.txt`: the file at the root of a site that tells
search engines which addresses they may read, and where the sitemap is.
The user chose to add it, in a card of its own, separate from the
sitemap's: whether the site needs one, and what it says, is a decision of
its own.

## Answers so far

### What does it say?

Chosen by the user: it points search engines to the sitemap, and it
blocks the robots that collect text to train artificial intelligence
models, while it lets through the robots that read a page when a user
asks, and those that index pages for search.

- **Why a `robots.txt` at all:** without one, search engines take
  everything as allowed (Google treats a missing `robots.txt`, answering
  404, as "no restrictions"), so the file is not needed for the site to
  be read. It is needed for the `Sitemap:` line, the only way a search
  engine finds the sitemap by itself; the other is submitting it by hand
  in each engine's console, such as Google Search Console or Bing
  Webmaster Tools.
- **Why block training:** the articles are under CC BY 4.0, which allows
  copying and adapting, also for profit, but asks to credit the author;
  the curriculum is under CC BY-ND 4.0, which also forbids sharing
  changed versions ([licences](../../content/README.md)). A model trained
  on the text usually credits nobody.
- **Three kinds of robot, decided one by one.** Anthropic's documentation
  ([Does Anthropic crawl data from the web](https://support.claude.com/en/articles/8896518-does-anthropic-crawl-data-from-the-web-and-how-can-site-owners-block-the-crawler),
  read on 2026-11-07) names three, all honouring `robots.txt`:
  `ClaudeBot` collects text that may train models; `Claude-User` reads a
  page when a user asks Claude to; `Claude-SearchBot` indexes pages for
  search. OpenAI has the same three kinds (`GPTBot`, `ChatGPT-User`,
  `OAI-SearchBot`). Blocking `ClaudeBot` only keeps the site's future
  text out of training; Claude can still read the site when a user asks.
- **`Google-Extended` is blocked,** by the user's choice: "training
  matters more". It is not a robot but a name for `robots.txt` that
  decides how Google may use what its ordinary robot already read, and
  it covers two things the user treats differently: training the Gemini
  models, and grounding, where Gemini is given text from Google's search
  index at the time of a user's question, the equivalent of reading on
  request. Google's documentation
  ([Google's common crawlers](https://developers.google.com/search/docs/crawling-indexing/google-common-crawlers),
  read on 2026-11-07) says blocking it does not affect inclusion or
  ranking in Google Search. The price: Gemini's answers will not draw on
  the site's text.
- **A request, not a lock:** `robots.txt` is honoured by whoever chooses
  to; a browser, a script or a less careful robot can ignore it. Text
  collected before the rule stays where it is.
- **Rejected:** a `robots.txt` that only points to the sitemap, which
  leaves the use of the text for training open; blocking every robot of
  artificial intelligence, which would also keep the site out of the
  answers and searches made with those tools.

### Which training robots?

Chosen by the user: those of the list kept by the community project
[`ai-robots-txt/ai.robots.txt`](https://github.com/ai-robots-txt/ai.robots.txt),
filtered to leave out the robots that read a page on a user's request
and those that index pages for search.

- **The list,** read on 2026-11-07: MIT licence, about 4,200 stars, last
  updated on 2026-11-05, 204 robots in `robots.json`, each with a
  free-text `function`. 147 fall into seven recurring categories: "AI
  Assistants" (31, such as `Claude-User`, `ChatGPT-User`), "AI Search
  Crawlers" (14, such as `OAI-SearchBot`), "AI Data Scrapers" (21), "AI
  Data Providers" (20), "AI Agents" (19), "AI Coding Agents" (9),
  "Undocumented AI Agents" (8); the other 57 have descriptions of their
  own, such as "LLM training." or "Data is sold.".
- **Why filtered:** the project's own `robots.txt` blocks all 204,
  reading on request and search included, against the user's choice.
- **Rejected:** a short list written by the project, clear but left
  behind by new robots and updated by hand; blocking everyone and
  allowing only chosen robots, which would also keep out a new search
  engine until it is added.

## Known from the start

- **There is nothing to disallow today.** The "page not found" is kept
  out of search results by `noindex`. Disallowing it in `robots.txt`
  would not help: Lighthouse counts a page disallowed there as blocked
  too, so its rule on indexing would fail anyway; and search engines
  could no longer read the page, so they would not see its `noindex`, as
  Google warns
  ([card](../wip/check-seo-with-lighthouse.md), "How is a page
  exempted?").
- **Its main use here** would be the `Sitemap:` line, pointing search
  engines to the [sitemap](../done/2026-11-07_add-sitemap.md).
- **Lighthouse's rule on `robots.txt`** is "not applicable" today, since
  the file is missing; with the file it becomes active and checks it is
  valid. That change is one of those the card
  [watch the rules Lighthouse finds not applicable](../backlog/watch-lighthouse-rules-not-applicable.md)
  is about.
- **It must be at the root of the host,** `/robots.txt`; the site is
  served from the root of `https://www.lucalorenzon.com/` since
  2026-11-01.

## Questions

- How the filter is made: which categories are let through, and how
  the robots with a description of their own are classed.
- Where the list is taken from and at which version, and how it is kept
  up to date.
- How the build writes it, and how the test checks it.
