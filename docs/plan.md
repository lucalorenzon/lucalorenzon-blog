# Plan

Last updated: 2026-11-02

The project's board: what is being worked on now, what comes next, what
is known or wished for later. How it is used: `CLAUDE.md`, "The plan".

## Goal

A personal blog that showcases the author's professional experience
through articles. It grows by small increments from a working skeleton,
live since 2026-10-26
([card](done/2026-10-26_put-working-skeleton-live.md)).

## Now

- [Serve the site's own "page not found"](backlog/serve-own-not-found-page.md),
  instead of GitHub's, now that the site lives at
  `https://www.lucalorenzon.com/`
  ([card](done/2026-11-01_choose-site-address.md)).
- [Cover the WCAG criteria axe cannot check](backlog/cover-wcag-criteria-axe-cannot-check.md),
  criterion by criterion, levels A, AA and AAA, now that readability
  is done ([card](done/2026-11-02_improve-site-readability.md)): for
  each one, whether it is done now, next or later.
- [Choose the licence of the articles](backlog/choose-articles-licence.md),
  before the two articles below, as its card asks.
- [Add the two articles already on Medium](backlog/add-the-two-medium-articles.md),
  copied under `content/` for now: the site's first articles.

## Next

- [Add site navigation](backlog/add-site-navigation.md), after the two
  articles already on Medium, when the home page becomes the list of
  contents.
- [Use the space beside the text on wide screens](backlog/use-space-beside-text-on-wide-screens.md),
  left empty by the narrower column of version 0.5.0.
- [Check performance in the build](backlog/check-performance-in-the-build.md),
  Lighthouse or similar, so a choice that makes the site slower or
  heavier does not go unnoticed.
- [Observe the visitors in production](backlog/observe-visitors-in-production.md):
  who reads the site, what they look for, where they interact; with
  the consent and the cookie policy it brings.
- [Update the screen reader check for an English site](backlog/update-screen-reader-check-for-english-site.md),
  since its reference still describes the Italian page.
- [Check the declared languages in the build](backlog/check-declared-languages-in-the-build.md),
  the part of the screen reader check that depends on the site and the
  accessibility tree does not cover.
- [Run a full round of `residuality`](backlog/run-full-residuality-round.md).

## Later

- [Give the screen reader container an Italian voice](backlog/give-the-screen-reader-container-an-italian-voice.md),
  probably superseded now that the site is in English: decided by the
  screen reader card in Next.
- [Run the browser tests in more browsers](backlog/run-browser-tests-in-more-browsers.md),
  Firefox and WebKit besides Chromium.
- [Adapt the footer and the articles to screen width](backlog/adapt-footer-and-articles-to-screen-width.md),
  with the rules for long titles already agreed.
- [Place the 1985 photo](backlog/place-1985-photo.md), not on the home
  page.
- [Take the site's content from several repositories](backlog/take-content-from-several-repositories.md),
  once copying the curriculum and the articles by hand stops scaling.
- [Refresh the stylesheet after a new version](backlog/refresh-stylesheet-after-a-new-version.md),
  so a returning visitor does not keep the old look.
- [Check the workflows with actionlint](backlog/lint-workflows-with-actionlint.md),
  in the pipeline rather than on one machine.
- [Refer to cards by name in frozen documents](backlog/refer-to-cards-by-name-in-frozen-documents.md),
  so moving a card no longer breaks links in minutes and finished cards.
- [Show the check results as badges in the README](backlog/show-check-results-as-readme-badges.md),
  from shields.io, after the performance check.
- [Measure the CO2 of a page view](backlog/measure-co2-of-a-page-view.md),
  an estimate from the bytes transferred.
- Further articles and everything around them, beyond the first two in
  Now. Not planned yet, on purpose.
