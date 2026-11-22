# Plan

Last updated: 2026-11-22

The project's board: what is being worked on now, what comes next, what
is known or wished for later. How it is used: `CLAUDE.md`, "The plan".

## Goal

A personal blog that showcases the author's professional experience
through articles. It grows by small increments from a working skeleton,
live since 2026-10-26
([card](cards/put-working-skeleton-live.md)).

## Now

- The round of the refactors left waiting is done, not published yet:
  the errors declared with thiserror, started in the domain and printed
  only by the build
  ([card](cards/refactor-errors-with-thiserror.md)), after
  jiff's day for the dates
  ([card](cards/give-dates-a-module-of-their-own.md)) and
  the articles' front matter
  ([card](cards/read-article-fields-from-front-matter.md)).
- [Build a design system](cards/build-a-design-system.md): a style
  guide of our own, built with Leptos and published under
  `/style-guide`, one page per component: each component tested once,
  on cases written for the purpose, so the two articles that serve as
  test pages today go back to being content. It takes over the components' test pages of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md),
  whose other slices are done: 913 tests became 227.
  - First page done, not published yet:
    [show the colours in the style guide](cards/show-the-colours-in-the-style-guide.md),
    defined once in the build, which writes the stylesheet's values
    and checks every allowed pair's contrast.
  - The pages left before publishing, in this order: the code block
    with its wrap toggle
    ([card](cards/show-the-code-block-in-the-style-guide.md),
    done; each page links to the index and back to
    its top, done
    ([card](cards/link-the-style-guide-pages-to-its-index.md)),
    and the wrap toggle turned into an icon, done
    ([card](cards/turn-the-code-wrap-toggle-into-an-icon.md))),
    the table with its record viewer, done
    ([card](cards/show-the-table-in-the-style-guide.md)),
    the typography, cut in two: the elements of the text, done
    ([card](cards/show-the-text-elements-in-the-style-guide.md)),
    so no browser test uses an article as a test page any more, with
    a fix it brought up, the accessibility check made to scroll the
    page before measuring, done
    ([card](cards/scroll-before-the-accessibility-check.md)),
    and the fonts, done
    ([card](cards/show-the-fonts-in-the-style-guide.md));
    then the fonts' cards below; last, the breakpoints.
  - The fonts made one setting for the whole site, first in the
    stylesheet, done
    ([card](cards/give-the-fonts-three-roles-in-the-stylesheet.md)):
    three roles, text, code and logo, so a change of font there is a
    change of every page; then written by the build from a
    configuration file, done
    ([card](cards/link-the-fonts-to-the-build.md)); then
    the fonts page built from the configuration, one section per font
    with its roles, done
    ([card](cards/build-the-fonts-page-from-the-configuration.md)).
    The wrap toggle hidden on blocks that fit, which Chromium on Linux
    drew narrower than the build assumes, done
    ([card](cards/hide-the-wrap-toggle-on-blocks-that-fit.md)),
    with the cases at the limit of the wrap toggle on the code page as
    its regression test; and the proof that those cases stop the build
    when a code font moves the threshold, done
    ([card](cards/measure-the-fonts-again-when-they-change.md)).
    The breakpoints, their values read from `config/` by the build and
    the tests, done
    ([card](cards/read-the-breakpoints-from-the-configuration.md));
    their page superseded
    ([card](cards/show-the-breakpoints-in-the-style-guide.md))
    by the finding of a spike, done
    ([card](cards/try-a-breakpoint-declaration-on-the-text-column.md)):
    a component's page shows what changes in it, nothing is declared.
    Still to do before publishing, one page of a component. The text
    column, made a component, done
    ([card](cards/make-the-text-column-a-component.md)), went back
    into the page frame, which writes the `<main>` again, done
    ([card](cards/decide-whether-to-cut-the-text-column-again.md));
    so the column's page joined the frame's, closed
    ([card](cards/show-the-text-column-in-the-style-guide.md)). First
    the column's text made to grow within a range the site injects
    ([card](cards/let-the-column-text-grow-within-an-injected-range.md),
    done, not published yet: 18 to 20 pixels between 320 and 1366,
    the range written in the stylesheet),
    then the frame's text within ranges of its own over the same widths
    ([card](cards/let-the-frame-text-grow-within-the-injected-ranges.md),
    done, not published yet: the title from 24 to 32 pixels, the
    footer from 16 to 18, the section headings at 1.2 times the text),
    then the frame's page, which shows the column too
    ([card](cards/show-the-page-frame-in-the-style-guide.md),
    done, not published yet: its text, a draft by the agent for the
    user to rewrite, lives beside the frame's code, as
    [ADR-005](../adr/ADR-005-explain-a-component-beside-its-code.md)
    says), then
    the frame's tests moved onto it from the home page
    ([card](cards/move-the-frame-tests-to-its-page.md), done, the
    home page left to the net) and the
    column's from the text page
    ([card](cards/move-the-column-tests-to-the-frame-page.md), done,
    with the headings' ratios),
    and the index's list of
    the pages to come, closed: the index lists only the pages that
    exist
    ([card](cards/declare-the-pages-to-come-in-the-style-guide-index.md)).
    Last, before publishing, the author's texts in place of the
    placeholders: the frame's page, a section of the colours page, and
    the frame shown as one piece for now
    ([card](cards/write-the-style-guide-placeholder-texts.md)).
- [Review the logo](cards/review-the-logo.md), done, not published
  yet: one icon, the favicon's photo cut to a circle, standing on the
  header's line, with the name beside it in Literata bold italic on
  the same line, in place of the written logo and the logo drawn in
  characters; the role `logo` moved to Literata.
  What it left, after it: first
  the line hidden under the name like a thread
  ([card](cards/hide-the-header-line-under-the-logo-name.md)),
  the one remark of the user on the screenshots, done, not published
  yet;
  then the pixel font's files and ADR-003, done, not published yet:
  the files gone with the fonts' native size, and ADR-006 in place of
  ADR-003's part on the logo
  ([card](cards/remove-the-pixel-font.md));
  the favicon made from the logo's crop, done, not published yet:
  one icon file of 16 and 32 pixels
  ([card](cards/make-the-favicon-from-the-logo-crop.md)),
  and whether the logo is a component of its own
  ([card](cards/decide-whether-the-logo-is-a-component.md)).
- [Empty the site folder before the build](cards/empty-the-site-folder-before-the-build.md),
  in `scripts/build-site.sh`, so a file the site no longer has does
  not stay in `target/site` from an earlier build; done, not published
  yet.

## Next

- [Update the screen reader check for an English site](cards/update-screen-reader-check-for-english-site.md),
  since its reference still describes the Italian home page, and let
  it read the pages that make sense, probably the style guide's
  rather than the home page: it carries the wrap toggle card's
  acceptance test on the screen reader.
- [Find cases at the limit for the growing column](cards/find-cases-at-the-limit-for-the-growing-column.md),
  in place of the tables' cases at the limit, taken out when the
  column's text stopped jumping at 768 pixels.
- [Check the table widths against Chromium's rounding](cards/check-table-widths-against-chromium-rounding.md),
  which drew the code font a whole number of pixels wide on Linux and
  does the same with the column's text: the column steps there, and
  the six-column table changes form more than once as the page
  widens.
- [Follow a known device list in the browser tests](cards/follow-a-known-device-list-in-the-tests.md),
  perhaps Firefox's, and give the tests' 320 a name that is not the
  iPhone SE the user looks at.
- [Take the measures from the configured fonts](cards/take-the-measures-from-the-configured-fonts.md),
  in the build, in the browser, instead of Literata's widths generated
  by hand and Hack's advance written in the code.
- [Read the column text's range from the configuration](cards/read-the-column-text-range-from-the-configuration.md),
  the second increment of the column's text, after the range is
  written in the stylesheet.
- [Use a structured configuration file](cards/use-a-structured-configuration-file.md),
  now that `config/` holds a second setting, the fonts, read by the
  build only; the heading limit is read by the tests too.
- [Move the style guide's texts beside their code](cards/move-the-style-guide-texts-beside-their-code.md),
  as the frame's page will have it, after the style guide is
  published.
- [Write an article on the style guide](cards/write-an-article-on-the-style-guide.md),
  told from the problem it solves, once the style guide is complete:
  published with it, as the third article, and the readers' way into
  it.
- [Separate the domain from the adapters](cards/separate-the-domain-from-the-adapters.md),
  after the errors: a core of pure business that returns its errors,
  and one adapter per concept the file system implements today
  (content source, publication folder, configuration).
- [Let Tailwind read only the presentation](cards/let-tailwind-read-only-the-presentation.md),
  not every word of the Rust code, which can become a rule of the
  stylesheet; some answers do not wait for the separation above.
- [Unify the model of the information pages](cards/unify-the-information-pages-model.md),
  licence, about and curriculum, today three nearly identical modules;
  probably before reading their fields from the Markdown.
- [Let a page state its own licence terms](cards/let-a-page-state-its-own-licence-terms.md),
  departing from the licence page, as the SAP articles' closing note
  does today in the text; after the information pages' model.
- [Read the other pages' title and description from the Markdown](cards/read-page-fields-from-markdown.md),
  the curriculum, about and licence pages, and where the home page's
  description goes, after the articles.
- [Take the pages' addresses from a slug, with redirects](cards/take-page-addresses-from-slug.md),
  so an address can follow the marketing campaigns without breaking
  the old one.
- [Find the articles in the content folder](cards/find-articles-in-content-folder.md)
  when the build runs, instead of a list of files wired into the code.
- [Check the pages' text in the build](cards/check-page-bodies-in-the-build.md):
  not empty, without HTML and without empty table headers, with the
  same rule as a wrong field, instead of HTML checked by unit tests
  only.
- [Give the home page a date in the sitemap](cards/date-the-home-page-in-the-sitemap.md):
  the date of the last addition or removal of an article.
- [Use the publication date in the sitemap](cards/use-publication-date-in-the-sitemap.md),
  instead of the day a change was written, without publishing something
  other than what was tested.
- [Cover the WCAG criteria axe cannot check](cards/cover-wcag-criteria-axe-cannot-check.md),
  criterion by criterion, levels A, AA and AAA, now that readability
  is done ([card](cards/improve-site-readability.md)): for
  each one, whether it is done now, next or later.
- [Add icons for the home screen](cards/add-icons-for-the-home-screen.md)
  of phones, cut from the favicon's card, with the
  [decision whether to keep the logo images' script](cards/decide-whether-to-keep-the-logo-images-script.md),
  which would make them.
- [Add preview images](cards/add-preview-images.md) for links shared
  on LinkedIn or elsewhere, one per page, in a session of their own,
  probably after the logo is reviewed.
- [Add a menu for the site](cards/add-site-menu.md), in the room the
  header keeps on the right: search, categories, subscribing,
  settings, after the list of articles is published.
- [Add breadcrumbs and a way back to the top to the articles](cards/add-breadcrumbs-and-back-to-top-to-articles.md),
  reusing the ones the style guide gets first.
- [Use the space beside the text on wide screens](cards/use-space-beside-text-on-wide-screens.md),
  left empty by the narrower column of version 0.5.0.
- [Check performance in the build](cards/check-performance-in-the-build.md),
  so a choice that makes the site slower or heavier does not go
  unnoticed; Lighthouse is already in the build, with the SEO check.
- [Observe the visitors in production](cards/observe-visitors-in-production.md):
  who reads the site, what they look for, where they interact; with
  the consent and the cookie policy it brings.
- [Check the declared languages in the build](cards/check-declared-languages-in-the-build.md),
  the part of the screen reader check that depends on the site and the
  accessibility tree does not cover.
- [Run a full round of `residuality`](cards/run-full-residuality-round.md).

## Later

- [Give the screen reader container an Italian voice](cards/give-the-screen-reader-container-an-italian-voice.md),
  probably superseded now that the site is in English: decided by the
  screen reader card in Next.
- [Run the browser tests in more browsers](cards/run-browser-tests-in-more-browsers.md),
  Firefox and WebKit besides Chromium.
- [Adapt the footer and the articles to screen width](cards/adapt-footer-and-articles-to-screen-width.md),
  with the rules for long titles already agreed.
- [Place the 1985 photo](cards/place-1985-photo.md), not on the home
  page.
- [Publish the curriculum in more languages](cards/publish-curriculum-in-more-languages.md),
  translated by the author, since its licence forbids others to share
  translations.
- [Take the site's content from several repositories](cards/take-content-from-several-repositories.md),
  once copying the curriculum and the articles by hand stops scaling.
- [Check periodically that the published text matches its Markdown](cards/check-published-text-against-markdown.md),
  so a section lost while a page is composed does not go unnoticed,
  now that the tests no longer write the section titles.
- [Break long words in the text](cards/break-long-words-in-the-text.md),
  so no text can scroll the page sideways and that check can leave the
  net for a component test.
- [Refresh the stylesheet after a new version](cards/refresh-stylesheet-after-a-new-version.md),
  so a returning visitor does not keep the old look.
- [Check the workflows with actionlint](cards/lint-workflows-with-actionlint.md),
  in the pipeline rather than on one machine.
- [Show the check results as badges in the README](cards/show-check-results-as-readme-badges.md),
  from shields.io, after the performance check.
- [Add a second way to download Tailwind](cards/add-a-second-way-to-download-tailwind.md),
  through GitHub's programming interface, if the build stops again on
  its download despite the cache and the retries.
- [Watch the rules Lighthouse finds not applicable](cards/watch-lighthouse-rules-not-applicable.md),
  which the SEO check counts as passed, so a rule that stops or starts
  applying does not go unnoticed.
- [Check the structured data](cards/check-structured-data.md), which
  Lighthouse leaves to a person: whether the site needs any, and how to
  check it.
- [Port the Python scripts to Rust, run with Docker](cards/port-python-scripts-to-rust-with-docker.md):
  the screen reader check and the sitemap's dates step.
- [Measure the CO2 of a page view](cards/measure-co2-of-a-page-view.md),
  an estimate from the bytes transferred.
- [Let readers adjust font and spacing](cards/let-readers-adjust-font-and-spacing.md),
  as on a Kindle, and choose the background on the site itself.
- [Break wrapped code lines by grammar](cards/break-wrapped-code-by-grammar.md),
  rather than wherever the width runs out.
- [Add syntax colours to code](cards/add-syntax-colours-to-code.md),
  within the contrast the site checks; it may share a tool with the
  grammar breaks.
- [Handle long tables](cards/handle-long-tables.md): a header that
  stays in view, and the record viewer's copies, which grow with the
  square of the records; with the first article that has a long table.
- [Measure what a Leptos island weighs](cards/measure-weight-of-a-leptos-island.md),
  the shared part and the component's, with the first island the site
  needs.
- [Add a copy button to code blocks](cards/add-a-copy-button-to-code-blocks.md),
  the site's first script: an island with leptos-use's `use_clipboard`,
  or plain JavaScript; probably the island above.
- [Run a browser test from its row in the README](cards/run-a-test-from-its-readme-row.md),
  on the last build, without writing the command by hand.
- [Measure the code font on macOS](cards/measure-the-code-font-on-macos.md),
  in Safari and Chrome, since the wrap toggle's threshold was checked
  on Linux only.
- [Launch the site on LinkedIn](cards/launch-the-site-on-linkedin.md),
  the real launch, once the visitors are observed and a few more
  articles are online.
- [Plan marketing campaigns](cards/plan-marketing-campaigns.md),
  perhaps with banners, after the user has learned about them.
- Further articles and everything around them, beyond the first two,
  now on the site. Not planned yet, on purpose; the launch's card
  names the ones the user has in mind.
