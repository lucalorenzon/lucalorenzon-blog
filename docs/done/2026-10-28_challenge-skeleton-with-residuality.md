# Challenge the live skeleton with `residuality`

- **Opened:** 2026-10-26
- **Status:** done, closed on 2026-10-28, after a light pass. The full
  analysis is left to the card
  [run a full round of `residuality`](../backlog/run-full-residuality-round.md).

## Why it was opened

The working rules (`CLAUDE.md`, "How the product grows") say that once
the first version is live, `residuality` stresses it, and the next
increments are decided from that outcome. The skeleton went live on
2026-10-26: before building on top of it, the question was which choices
made for a one-page site would break first as the site changes.

## Answers

### How deep should the analysis go?

**Chosen: a light pass, one round of questions**, only to find the next
increment that creates value. The full method (incidence matrix,
trade-offs, failure analysis, empirical test) is left to a later round.

Why: the project's rules ask to find what to do right away rather than
settle every detail in advance. The full method is long, and its value
grows once there is more structure to stress than a single page.

Rejected: the full eight-step analysis in this session, for the reason
above.

### What was stressed?

The site as it stood on 2026-10-28, not as the card described it on
2026-10-26: the pipeline had changed since, with the build and the
publication separated
([card](2026-10-28_separate-build-from-deploy.md)).

- one Leptos component written by hand (`src/home_page.rs`), turned into
  a single `index.html` by the build program (`src/main.rs`);
- Tailwind compiled by its standalone tool inside
  `scripts/build-site.sh`, which runs only on Linux x64;
- a Playwright test, run on the freshly built site and again on the
  published one;
- the build workflow, which on every push to `main` keeps the tested site
  in the `snapshot` Release;
- the deploy workflow, started by a `vX.Y.Z` tag, which requires that
  version's section in `CHANGELOG.md` and publishes to GitHub Pages under
  the sub-path `/lucalorenzon-blog/`.

### Which stressors were raised?

The agent proposed six, each anchored to something in the site; the
user found them too technical and raised a seventh, about the site's
purpose.

| # | Stressor | Where it touches today | What would happen |
|---|---|---|---|
| 1 | The first article arrives, then ten, then fifty | `main.rs` writes only `index.html`, and the page is Rust code | Every article would need Rust code; there is no place where an article is just text |
| 2 | An article is to be published without a new software version | Publication starts only from a version tag and requires a CHANGELOG section | Every article becomes a version; the CHANGELOG mixes site changes and content |
| 3 | A page links to another | Only relative paths today, and the public address is still a sub-path | Every link depends on an address choice not made yet |
| 4 | The project is left alone for six months, then picked up again | About 560 lines of GitHub Actions workflows; pinned Leptos, Tailwind, Playwright and Rust versions | The publication has to be relearned, and the tools may have aged |
| 5 | GitHub changes Pages or Actions, or the site leaves GitHub | Pages, Actions and the `snapshot` Release are all on GitHub | The site itself is static files and moves easily; the pipeline would have to be rewritten |
| 6 | Extreme case: one article gets a hundred thousand visits in a day | Static pages on GitHub Pages | Probably holds as it is |
| 7 | A visitor arrives today (raised by the user) | The page shows only "Sito in costruzione"; the author's name appears only in the browser tab title and, indirectly, in the address | The visitor cannot tell whose site it is. The skeleton holds technically, but misses the reason the site exists |

### Which stressor drives the next increment?

**Chosen: stressor 7.** The site's outcome is that its content is tied
to its author; an "under construction" page could belong to anyone. This
comes before articles: an article without a recognisable author stays
as anonymous as today's page.

What a visitor should find, as the user described it:

- the audience is mostly technical people who appreciate simplicity that
  does not become trivial;
- a minimal style, dark, like an old terminal. It tells where the author
  comes from: the user has photos of themselves aged twelve, in 1985, in
  a server room in front of a terminal. The photo explains the style; it
  is not for the home page;
- a header with the ASCII-art logo, a footer with a copyright line, a
  favicon. Logo and favicon already exist on the branch `old_main`.

The residue, in `residuality`'s terms: the structure stays as it is; the
page gains what ties it to its author. It became the card
[give the page a header, a footer and a favicon, in a dark terminal style](../backlog/add-header-footer-and-favicon.md).

How it was cut: the agent first proposed four slices (frame, dark
style, favicon, the author's story). The user judged that frame, style
and favicon are one slice, since the style is a stylesheet and the
favicon an existing file, and that the photo is not home-page content.

Stressors 1 to 6 were not acted on; they are the starting material of
the full round.

### What else came out?

- **Licence of the content.** The whole repository is under MIT, so
  future articles would be too. The user keeps MIT for now and will
  consider Creative Commons later: card
  [choose the licence of the articles](../backlog/choose-articles-licence.md).
- **The 1985 photo.** Where to place it is to be decided, not on the home
  page: card [place the 1985 photo](../backlog/place-1985-photo.md).
- **The year in the footer.** The site is generated at publication, so
  the year stays the one of the last published version. Noted in the
  header and footer card, not acted on.

## Input for the full round

Things noticed while building the skeleton, still to be stressed:

- **Static generation.** The build program was chosen over `leptos_axum`'s
  built-in static generation because it is thinner for one page
  ([card](2026-10-26_put-working-skeleton-live.md)). How it holds up with
  many pages and articles is an open question (stressor 1).
- **Site address.** Sub-path or custom domain is still open
  ([card](../backlog/choose-site-address.md)); it affects every link
  (stressor 3), and also how much the address itself ties the site to
  its author (stressor 7).
- **Single platform build.** `scripts/build-site.sh` supports only Linux
  x64, the platform of both the author's machine and the pipeline.
- **Leptos dependencies.** `paste`, pulled in by Leptos, is reported as
  unmaintained (RUSTSEC-2024-0436); no safe upgrade exists at this layer.
- **Version alignment.** Leptos 0.9 is at release-candidate stage; GitHub
  is moving `ubuntu-latest` to Ubuntu 26.

No longer an input: the browser test's dependence on Ubuntu's package
server, removed on 2026-10-27
([card](2026-10-27_run-browser-tests-in-playwright-image.md)).
