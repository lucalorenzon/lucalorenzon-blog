# Check accessibility in the build

- **Opened:** 2026-10-29, from the split of the card "Add automated
  accessibility tests", opened on 2026-10-28 by the user while the card
  [give the page a header, a footer and a favicon](2026-10-28_add-header-footer-and-favicon.md)
  was being worked on.
- **Status:** done on 2026-10-30, in commit `9b259f9`.

## Why it was opened

On the previous attempt of the site the user had ruled out colours other
than white for accessibility reasons. For the dark terminal style they
wanted to try amber on black. The contrast of the chosen amber,
`#FFB000` on `#000000`, was computed on 2026-10-28 at 11.46:1, above the
WCAG thresholds for normal text (4.5:1 for AA, 7:1 for AAA).

That was a one-off check by hand. Every later change of colour, layout
or content could break accessibility without anyone noticing: the build
should check it every time, the way the browser test already checks
what a visitor sees.

## Answers

### Which tool?

**axe-core, through `@axe-core/playwright` 4.14.0, inside the existing
Playwright test.** It is the most widely used accessibility scanner and
needs no new test runner. Its licence, MPL-2.0, applies file by file,
and the package is used only by the tests, so it never reaches the
published site.

### Which level, and what happens on a violation?

**Levels A and AA of WCAG 2.0, 2.1 and 2.2, plus the three AAA rules axe
can check; any violation fails the build (chosen by the user).** The
original card had a separate card planned for the AAA level; the user
asked to fold it in, since for axe a level is only a list of rules in
the same check. axe 4.14 checks three AAA rules: enhanced contrast (at
least 7:1 for normal text, 4.5:1 for large text), links with the same
text leading to the same purpose, no automatic page refresh. They are
off by default; selecting them by tag turns them on.

What this commits the site to: every future colour must reach 7:1 on
normal text. What it does not mean: the site is not "AAA", since most
AAA criteria can only be checked by a person; that is the card
check accessibility by hand, since split into
[check the accessibility tree in the build](../wip/check-accessibility-tree-in-the-build.md)
and [run a screen reader check on demand](../backlog/run-screen-reader-check-on-demand.md).

### At which widths?

**One per layout band: 393, 768 and 1280 pixels**, the same sizes the
header layout test uses. The page shows different elements per band
(text logo, ASCII-art logo, title beside the logo), and axe checks only
what is shown.

### How do we know the check is not empty?

The test requires the two contrast rules to appear among the rules axe
evaluated and passed, so an empty list of violations cannot come from
rules that never ran. And the check was seen failing: in a copy of the
site with the amber lowered to `#b07a00` (5.63:1, enough for AA, not for
AAA), the three tests failed on enhanced contrast only, for the body
and footer text; the title and the logo passed, as large text needs
4.5:1.

## Results

- In the Playwright 1.64.1 image: type check with no errors, 16 browser
  tests out of 16 passed.
- The same test also runs on the live site after each publication.
