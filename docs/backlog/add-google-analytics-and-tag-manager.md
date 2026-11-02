# Add Google Analytics 4 and Google Tag Manager

- **Opened:** 2026-11-02, by the user.
- **Status:** noticed, not started; in Next.

## Why

The user wants Google Analytics 4 (GA4), managed through Google Tag
Manager (GTM), on the site.

## Known from the start

- **Today the site sends nothing to third parties** and runs no
  JavaScript: everything, fonts included, is served by the site itself.
  Fonts from Google Fonts were rejected on 2026-10-28 for visitors'
  privacy (fines in Europe)
  ([card](../done/2026-10-28_add-header-footer-and-favicon.md)). GA4
  sends data about visitors to Google, so the same concern applies; an
  earlier choice to weigh, not a rule that forbids it.
- **Consent.** In Europe, analytics cookies such as GA4's generally
  require the visitor's consent before they are set, so a consent
  banner, or GA4's consent mode, comes with it. In 2022 the Italian
  data protection authority ruled the then Google Analytics unlawful
  because of transfers to the United States; the EU-US Data Privacy
  Framework of July 2023 changed the legal basis since. Both to be
  checked against current rules before choosing.
- **Other cards are touched:**
  - [check performance in the build](check-performance-in-the-build.md):
    GTM and GA4 add JavaScript and requests to every page;
  - [measure the CO2 of a page view](measure-co2-of-a-page-view.md):
    more bytes per visit;
  - [cover the WCAG criteria axe cannot check](cover-wcag-criteria-axe-cannot-check.md):
    a consent banner must be accessible (keyboard, focus, screen
    reader);
  - the browser tests run on every build and on the live site: they
    must not be counted as visits.
- **The repository is public:** a GTM container or GA4 measurement ID
  is public by nature, but the card should say so.

## Questions

- What the user wants to learn from the data (visits, pages read, where
  readers come from), which sets what is collected.
- Whether GTM is needed now, or GA4 alone is enough while the site has
  one page.
- How consent is asked and recorded, and what happens when it is
  refused.
- Whether a tool without cookies would answer the same questions with
  less weight and no banner.
- How the test visits of the build and of the live check are kept out
  of the numbers.
