# Changelog

What each published version of the site brought. The layout follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Rules for versions

A version is published by tagging a commit of `main` with `vMAJOR.MINOR.PATCH`
(how: [README](README.md#publish-a-version)). The publication fails if this
file has no section for that version.

What each number means for this site:

- **MAJOR:** a completely new design, or a different navigation.
- **MINOR:** an additional feature.
- **PATCH:** a security fix or a bug fix. While test articles live in this
  repository, adding one also counts as a patch.

While the site is at version 0, a new design counts as MINOR, not MAJOR:
the leading 0 says the site is still being built: the articles and the
navigation are still to come. Set on 2026-10-28, when the first design came with version 0.2.0
([card](docs/done/2026-10-28_add-header-footer-and-favicon.md)).

A version tag is never moved. If one has to be deleted, its name is never
used again: the next version takes the next number. GitHub can stop a tag
from being moved, but not a deleted name from being reused, so this rule
holds only because it is written here.

Why these rules: [the card that set them](docs/done/2026-10-28_separate-build-from-deploy.md).

## [Unreleased]

## [0.6.0] - 2026-11-02

### Added

- The site's own "page not found": an address that does not exist, at any
  depth, shows a page with the site's header and footer, the heading
  "Page not found" and a link to the home page, instead of GitHub's own
  page. The build takes the site's full address (`SITE_ADDRESS`, by
  default `https://www.lucalorenzon.com/`); only this page uses it, every
  other page keeps relative paths, so the site can later be served under
  a prefix without being rebuilt
  ([ADR-002](docs/adr/ADR-002-relative-paths-and-site-address-at-build.md)).

### Changed

- The home page's tab title puts the page's name first: "Curriculum Vitae
  - Luca Lorenzon". Every page follows that order.

## [0.5.0] - 2026-11-02

### Changed

- The text is easier to read: it is set in the Literata font, served by the
  site itself, with more space between lines (1.5 times the text size),
  between paragraphs and between list items, and a narrower column, so
  lines stay well under 80 characters. The amber is a little softer (`#DD9911`
  instead of `#FFB000`). The header keeps its old-terminal look and font.
  Browser tests now check the criteria of WCAG 1.4.8 (visual
  presentation) and 1.4.12 (text spacing).
- Configured `www.lucalorenzon.com` as the site's domain, and redirects to
  it from `lucalorenzon.com` and from `lucalorenzon.it`, with and without
  `www`. The former address `lucalorenzon.github.io/lucalorenzon-blog/`
  redirects there too. Done in the hosting and domain settings on
  2026-11-01, live before this version.

## [0.4.0] - 2026-11-01

### Added

- The home page shows the author's curriculum in place of "Sito in
  costruzione": profile, experience with the main work of each role,
  competencies, education, training and languages.

### Changed

- The site is in English. The page title is "Curriculum Vitae" instead
  of "Home Page".

## [0.3.1] - 2026-10-31

### Fixed

- Screen readers say the site's name on the ASCII-art logo again.
  Orca 51 read it as a block of code and skipped its name.

## [0.3.0] - 2026-10-29

### Added

- The header adapts to the width of the screen. On phones the logo
  becomes the text "Lvk@73r", in large letters, and the page title moves
  under the header line. From 768 pixels wide (tablets) the ASCII-art
  logo comes back. From 1280 pixels the title sits beside the logo,
  centred on the page while there is room.
- Room on the right of the header for the navigation menu still to
  come.

## [0.2.1] - 2026-10-28

### Fixed

- The browser test on the published site runs after every publication.
  With version 0.2.0, published from the snapshot, it was skipped without
  notice.

## [0.2.0] - 2026-10-28

### Added

- A header with the ASCII-art logo and the title "Home Page", and a
  footer with the copyright, always at the bottom of the window.
- An old-terminal look: amber text on black, in the IBM VGA 8x16 font
  served by the site itself.
- The favicon.

## [0.1.0] - 2026-10-28

### Added

- An "under construction" home page: the first version of the site.
- Publication from a version tag: changes merged into `main` are built and
  tested, but go live only when a version is tagged.
