# Plan

Last updated: 2026-11-19

The project's board: what is being worked on now, what comes next, what
is known or wished for later. How it is used: `CLAUDE.md`, "The plan".

## Goal

A personal blog that showcases the author's professional experience
through articles. It grows by small increments from a working skeleton,
live since 2026-10-26
([card](done/2026-10-26_put-working-skeleton-live.md)).

## Now

- The round of the refactors left waiting is done, not published yet:
  the errors declared with thiserror, started in the domain and printed
  only by the build
  ([card](done/2026-11-09_refactor-errors-with-thiserror.md)), after
  jiff's day for the dates
  ([card](done/2026-11-09_give-dates-a-module-of-their-own.md)) and
  the articles' front matter
  ([card](done/2026-11-08_read-article-fields-from-front-matter.md)).
- [Build a design system](wip/build-a-design-system.md): a style
  guide of our own, built with Leptos and published under
  `/style-guide`, one page per component: each component tested once,
  on cases written for the purpose, so the two articles that serve as
  test pages today go back to being content. It takes over the components' test pages of
  [ADR-004](adr/ADR-004-test-components-once-and-pages-with-a-net.md),
  whose other slices are done: 913 tests became 227.
  - First page done, not published yet:
    [show the colours in the style guide](done/2026-11-12_show-the-colours-in-the-style-guide.md),
    defined once in the build, which writes the stylesheet's values
    and checks every allowed pair's contrast.
  - The pages left before publishing, in this order: the code block
    with its wrap toggle
    ([card](done/2026-11-15_show-the-code-block-in-the-style-guide.md),
    done; each page links to the index and back to
    its top, done
    ([card](done/2026-11-14_link-the-style-guide-pages-to-its-index.md)),
    and the wrap toggle turned into an icon, done
    ([card](done/2026-11-15_turn-the-code-wrap-toggle-into-an-icon.md))),
    the table with its record viewer, done
    ([card](done/2026-11-15_show-the-table-in-the-style-guide.md)),
    the typography, cut in two: the elements of the text, done
    ([card](done/2026-11-16_show-the-text-elements-in-the-style-guide.md)),
    so no browser test uses an article as a test page any more, with
    a fix it brought up, the accessibility check made to scroll the
    page before measuring, done
    ([card](done/2026-11-16_scroll-before-the-accessibility-check.md)),
    and the fonts, done
    ([card](done/2026-11-16_show-the-fonts-in-the-style-guide.md));
    then the fonts' cards below; last, the breakpoints.
  - The fonts made one setting for the whole site, first in the
    stylesheet, done
    ([card](done/2026-11-16_give-the-fonts-three-roles-in-the-stylesheet.md)):
    three roles, text, code and logo, so a change of font there is a
    change of every page; then written by the build from a
    configuration file, done
    ([card](done/2026-11-16_link-the-fonts-to-the-build.md)); then
    the fonts page built from the configuration, one section per font
    with its roles, done
    ([card](done/2026-11-17_build-the-fonts-page-from-the-configuration.md)).
    The wrap toggle hidden on blocks that fit, which Chromium on Linux
    drew narrower than the build assumes, done
    ([card](done/2026-11-18_hide-the-wrap-toggle-on-blocks-that-fit.md)),
    with the cases at the limit of the wrap toggle on the code page as
    its regression test; and the proof that those cases stop the build
    when a code font moves the threshold, done
    ([card](done/2026-11-18_measure-the-fonts-again-when-they-change.md)).
    Still to do before publishing: the breakpoints, cut in two: their
    values read from `config/` by the build and the tests, done
    ([card](done/2026-11-18_read-the-breakpoints-from-the-configuration.md)), then
    the page, in work
    ([card](wip/show-the-breakpoints-in-the-style-guide.md)), which
    waits for the text column made a component, done
    ([card](done/2026-11-19_make-the-text-column-a-component.md)), and shown in
    a page of its own, in work
    ([card](wip/show-the-text-column-in-the-style-guide.md)), which
    waits for a spike on how a component tells the build what changes
    in it at each breakpoint, in work
    ([card](wip/try-a-breakpoint-declaration-on-the-text-column.md)).
- [Update the screen reader check for an English site](backlog/update-screen-reader-check-for-english-site.md),
  since its reference still describes the Italian page, and let it
  read more pages than the home page: it carries the wrap toggle
  card's acceptance test on the screen reader; next.

## Next

- [Check the table widths against Chromium's rounding](backlog/check-table-widths-against-chromium-rounding.md),
  which drew the code font a whole number of pixels wide on Linux and
  may do the same with the tables' text.
- [Take the measures from the configured fonts](backlog/take-the-measures-from-the-configured-fonts.md),
  in the build, in the browser, instead of Literata's widths generated
  by hand and Hack's advance written in the code.
- [Use a structured configuration file](backlog/use-a-structured-configuration-file.md),
  now that `config/` holds a second setting, the fonts, read by the
  build only; the heading limit is read by the tests too.
- [Write an article on the style guide](backlog/write-an-article-on-the-style-guide.md),
  told from the problem it solves, once the style guide is complete:
  published with it, as the third article, and the readers' way into
  it.
- [Separate the domain from the adapters](backlog/separate-the-domain-from-the-adapters.md),
  after the errors: a core of pure business that returns its errors,
  and one adapter per concept the file system implements today
  (content source, publication folder, configuration).
- [Let Tailwind read only the presentation](backlog/let-tailwind-read-only-the-presentation.md),
  not every word of the Rust code, which can become a rule of the
  stylesheet; some answers do not wait for the separation above.
- [Unify the model of the information pages](backlog/unify-the-information-pages-model.md),
  licence, about and curriculum, today three nearly identical modules;
  probably before reading their fields from the Markdown.
- [Let a page state its own licence terms](backlog/let-a-page-state-its-own-licence-terms.md),
  departing from the licence page, as the SAP articles' closing note
  does today in the text; after the information pages' model.
- [Read the other pages' title and description from the Markdown](backlog/read-page-fields-from-markdown.md),
  the curriculum, about and licence pages, and where the home page's
  description goes, after the articles.
- [Take the pages' addresses from a slug, with redirects](backlog/take-page-addresses-from-slug.md),
  so an address can follow the marketing campaigns without breaking
  the old one.
- [Find the articles in the content folder](backlog/find-articles-in-content-folder.md)
  when the build runs, instead of a list of files wired into the code.
- [Check the pages' text in the build](backlog/check-page-bodies-in-the-build.md):
  not empty, without HTML and without empty table headers, with the
  same rule as a wrong field, instead of HTML checked by unit tests
  only.
- [Give the home page a date in the sitemap](backlog/date-the-home-page-in-the-sitemap.md):
  the date of the last addition or removal of an article.
- [Use the publication date in the sitemap](backlog/use-publication-date-in-the-sitemap.md),
  instead of the day a change was written, without publishing something
  other than what was tested.
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
- [Add breadcrumbs and a way back to the top to the articles](backlog/add-breadcrumbs-and-back-to-top-to-articles.md),
  reusing the ones the style guide gets first.
- [Use the space beside the text on wide screens](backlog/use-space-beside-text-on-wide-screens.md),
  left empty by the narrower column of version 0.5.0.
- [Check performance in the build](backlog/check-performance-in-the-build.md),
  so a choice that makes the site slower or heavier does not go
  unnoticed; Lighthouse is already in the build, with the SEO check.
- [Observe the visitors in production](backlog/observe-visitors-in-production.md):
  who reads the site, what they look for, where they interact; with
  the consent and the cookie policy it brings.
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
- [Publish the curriculum in more languages](backlog/publish-curriculum-in-more-languages.md),
  translated by the author, since its licence forbids others to share
  translations.
- [Take the site's content from several repositories](backlog/take-content-from-several-repositories.md),
  once copying the curriculum and the articles by hand stops scaling.
- [Check periodically that the published text matches its Markdown](backlog/check-published-text-against-markdown.md),
  so a section lost while a page is composed does not go unnoticed,
  now that the tests no longer write the section titles.
- [Break long words in the text](backlog/break-long-words-in-the-text.md),
  so no text can scroll the page sideways and that check can leave the
  net for a component test.
- [Refresh the stylesheet after a new version](backlog/refresh-stylesheet-after-a-new-version.md),
  so a returning visitor does not keep the old look.
- [Check the workflows with actionlint](backlog/lint-workflows-with-actionlint.md),
  in the pipeline rather than on one machine.
- [Refer to cards by name in frozen documents](backlog/refer-to-cards-by-name-in-frozen-documents.md),
  so moving a card no longer breaks links in minutes and finished cards.
- [Show the check results as badges in the README](backlog/show-check-results-as-readme-badges.md),
  from shields.io, after the performance check.
- [Add a second way to download Tailwind](backlog/add-a-second-way-to-download-tailwind.md),
  through GitHub's programming interface, if the build stops again on
  its download despite the cache and the retries.
- [Watch the rules Lighthouse finds not applicable](backlog/watch-lighthouse-rules-not-applicable.md),
  which the SEO check counts as passed, so a rule that stops or starts
  applying does not go unnoticed.
- [Check the structured data](backlog/check-structured-data.md), which
  Lighthouse leaves to a person: whether the site needs any, and how to
  check it.
- [Port the Python scripts to Rust, run with Docker](backlog/port-python-scripts-to-rust-with-docker.md):
  the screen reader check and the sitemap's dates step.
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
- [Add a copy button to code blocks](backlog/add-a-copy-button-to-code-blocks.md),
  the site's first script: an island with leptos-use's `use_clipboard`,
  or plain JavaScript; probably the island above.
- [Run a browser test from its row in the README](backlog/run-a-test-from-its-readme-row.md),
  on the last build, without writing the command by hand.
- [Measure the code font on macOS](backlog/measure-the-code-font-on-macos.md),
  in Safari and Chrome, since the wrap toggle's threshold was checked
  on Linux only.
- [Launch the site on LinkedIn](backlog/launch-the-site-on-linkedin.md),
  the real launch, once the visitors are observed and a few more
  articles are online.
- [Plan marketing campaigns](backlog/plan-marketing-campaigns.md),
  perhaps with banners, after the user has learned about them.
- Further articles and everything around them, beyond the first two,
  now on the site. Not planned yet, on purpose; the launch's card
  names the ones the user has in mind.
