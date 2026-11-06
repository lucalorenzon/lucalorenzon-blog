# Plan

Last updated: 2026-11-06

The project's board: what is being worked on now, what comes next, what
is known or wished for later. How it is used: `CLAUDE.md`, "The plan".

## Goal

A personal blog that showcases the author's professional experience
through articles. It grows by small increments from a working skeleton,
live since 2026-10-26
([card](done/2026-10-26_put-working-skeleton-live.md)).

## Now

- [Add a sitemap](backlog/add-sitemap.md), from which the SEO check
  takes its list of pages.
- [Add a robots.txt](backlog/add-robots-txt.md), pointing to the
  sitemap.
- [Check SEO with Lighthouse in the build](wip/check-seo-with-lighthouse.md),
  on hold until the sitemap and the `robots.txt` exist, then resumed.
- The next version publishes all three together with the canonical
  link, the descriptions and the Open Graph tags, already done
  ([card](done/2026-11-06_add-canonical-link-to-every-page.md),
  [card](done/2026-11-06_add-description-and-open-graph-tags.md)).

## Next

- [Cover the WCAG criteria axe cannot check](backlog/cover-wcag-criteria-axe-cannot-check.md),
  criterion by criterion, levels A, AA and AAA, now that readability
  is done ([card](done/2026-11-02_improve-site-readability.md)): for
  each one, whether it is done now, next or later.
- [Review the logo](backlog/review-the-logo.md): on the light page the
  logo drawn in characters is hard to read; the user leans towards the
  written logo everywhere.
- [Add preview images](backlog/add-preview-images.md) for links shared
  on LinkedIn or elsewhere, one per page, in a session of their own,
  probably after the logo is reviewed.
- [Add a menu for the site](backlog/add-site-menu.md), in the room the
  header keeps on the right: search, categories, subscribing,
  settings, after the list of articles is published.
- [Use the space beside the text on wide screens](backlog/use-space-beside-text-on-wide-screens.md),
  left empty by the narrower column of version 0.5.0.
- [Check performance in the build](backlog/check-performance-in-the-build.md),
  so a choice that makes the site slower or heavier does not go
  unnoticed; Lighthouse arrives first with the SEO check.
- [Observe the visitors in production](backlog/observe-visitors-in-production.md):
  who reads the site, what they look for, where they interact; with
  the consent and the cookie policy it brings.
- [Update the screen reader check for an English site](backlog/update-screen-reader-check-for-english-site.md),
  since its reference still describes the Italian page.
- [Check the declared languages in the build](backlog/check-declared-languages-in-the-build.md),
  the part of the screen reader check that depends on the site and the
  accessibility tree does not cover.
- [Run a full round of `residuality`](backlog/run-full-residuality-round.md).
- [Make the browser tests check behaviour, not articles](backlog/make-browser-tests-check-behaviour.md),
  so a new article does not mean editing the tests' lists of pages.

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
- [Publish the curriculum in more languages](backlog/publish-curriculum-in-more-languages.md),
  translated by the author, since its licence forbids others to share
  translations.
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
- [Watch the rules Lighthouse finds not applicable](backlog/watch-lighthouse-rules-not-applicable.md),
  which the SEO check counts as passed, so a rule that stops or starts
  applying does not go unnoticed.
- [Check the structured data](backlog/check-structured-data.md), which
  Lighthouse leaves to a person: whether the site needs any, and how to
  check it.
- [Measure the CO2 of a page view](backlog/measure-co2-of-a-page-view.md),
  an estimate from the bytes transferred.
- [Let readers adjust font and spacing](backlog/let-readers-adjust-font-and-spacing.md),
  as on a Kindle, and choose the background on the site itself.
- [Break wrapped code lines by grammar](backlog/break-wrapped-code-by-grammar.md),
  rather than wherever the width runs out.
- [Add syntax colours to code](backlog/add-syntax-colours-to-code.md),
  within the contrast the site checks; it may share a tool with the
  grammar breaks.
- [Handle long tables](backlog/handle-long-tables.md): a header that
  stays in view, and the record viewer's copies, which grow with the
  square of the records; with the first article that has a long table.
- [Measure what a Leptos island weighs](backlog/measure-weight-of-a-leptos-island.md),
  the shared part and the component's, with the first island the site
  needs.
- Further articles and everything around them, beyond the first two,
  now on the site. Not planned yet, on purpose.
