# Observe the visitors in production

- **Opened:** 2026-11-02, by the user, first as "add Google Analytics 4
  and Google Tag Manager".
- **Status:** noticed, not started. Not useful while the site has one
  page: written so it is not forgotten.

## Why

The site has no observability in production. The user does not know who
visits it, what they look for, or where they interact most: in other
words, does not know the site's readers. Google Analytics 4 (GA4) and
Google Tag Manager (GTM) were named because they are the tools the user
knows; any tool that answers the need will do.

While the site is a single page there is little to observe; it becomes
useful with articles and navigation.

## Known from the start

- **Today the site sends nothing to third parties** and runs no
  JavaScript: everything, fonts included, is served by the site itself.
  Fonts from Google Fonts were rejected on 2026-10-28 for visitors'
  privacy (fines in Europe)
  ([card](../done/2026-10-28_add-header-footer-and-favicon.md)). A tool
  that sends visitors' data to a third party raises the same concern; an
  earlier choice to weigh, not a rule that forbids it.
- **Consent and policies.** In Europe, analytics cookies such as GA4's
  generally require the visitor's consent before they are set, so a
  consent banner comes with them. Collecting data about visitors also
  calls for a cookie policy and a privacy notice on the site. In 2022
  the Italian data protection authority ruled the then Google Analytics
  unlawful because of transfers to the United States; the EU-US Data
  Privacy Framework of July 2023 changed the legal basis since. Both to
  be checked against current rules before choosing.
- **There are tools of different kinds:** full analytics suites such as
  GA4, often managed through a tag manager such as GTM; and lighter
  tools that count visits without cookies, which may need no banner.
  Which one fits depends on what the user wants to learn.
- **Other cards are touched:**
  - [check performance in the build](check-performance-in-the-build.md):
    a tracking script adds JavaScript and requests to every page;
  - [measure the CO2 of a page view](measure-co2-of-a-page-view.md):
    more bytes per visit;
  - [cover the WCAG criteria axe cannot check](cover-wcag-criteria-axe-cannot-check.md):
    a consent banner must be accessible (keyboard, focus, screen
    reader);
  - the browser tests run on every build and on the live site: they
    must not be counted as visits.
- **The repository is public:** any tracking identifier in the code is
  public too.

## Questions

- What the user wants to learn: who the readers are, what they look
  for, where they arrive from, which pages and parts they use. This sets
  what is collected and which kind of tool fits.
- Which tool: a full suite such as GA4, with or without GTM, or a
  lighter one without cookies.
- What the cookie policy and privacy notice say, where they live on the
  site, and how consent is asked, recorded and refused.
- How the visits of the build's tests and of the live check are kept
  out of the numbers.
- When: with the first articles, or with the navigation.
