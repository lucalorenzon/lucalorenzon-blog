# Put the curriculum on the home page

- **Opened:** 2026-10-31, by the user.
- **Status:** done on 2026-11-01.

## Why it was opened

The site's only page said "Sito in costruzione". Before the site moves
to its own domain ([card](../backlog/choose-site-address.md)), it should
show real content, so the clean address is first indexed on something
concrete and not on a site under construction. The author's curriculum
is that content.

The curriculum is a page of its own, not an article (decided by the user
on 2026-10-31), so the choice of the articles' licence
([card](2026-11-02_choose-articles-licence.md)) still comes before the
first article. The site stays one page, with no internal links, until
the navigation arrives ([card](2026-11-06_list-articles-on-home-page.md)).

## Answers

### How the Markdown becomes part of the page

**Chosen:** the Markdown is converted to HTML at build time with the
library `pulldown-cmark` 0.13 (only its `html` feature), embedded in the
build program with `include_str!`, and placed in the page's `<main>`.
The conversion lives in its own module (`src/markdown.rs`), so the
articles can reuse it.

**Why:** the user wanted a library rather than a conversion by hand, so
the Markdown stays the text of reference. `pulldown-cmark` is small,
follows the CommonMark standard and is the library rustdoc and mdBook
use. Its licence is MIT; it brings three small crates (MIT or Apache).

**No HTML inside the Markdown:** the user wants to avoid it, so a unit
test fails if the curriculum contains any.

**Rejected:** converting by hand into the page component (the Markdown
would stop being the text of reference); `comrak` (heavier, and GitHub's
extensions to Markdown are not needed).

### Where the text comes from

**Chosen:** a version written for the site, copied into
`content/curriculum.md`. Its sources: the author's private curriculum
repository (`cv-lucalorenzon`, commit `d5093b1`), the projects on the
author's LinkedIn profile, and what the author added during the session.

**Why:** fetching the text from the private repository during the build
would bring in permissions and credentials, too much for this card. The
question is kept in its own card
([take the site's content from several repositories](../backlog/take-content-from-several-repositories.md)).
The existing versions were written for automatic candidate screening
(ATS); the site deserved its own.

**Rejected:** a git submodule pointing to the curriculum repository (the
build would need credentials, and the file published would not be the
cleaned one); a comment in the file naming its source (it would be HTML
inside the Markdown; the source is in the commit message instead).

### What is made public

**Chosen:**

- No email, phone number or address. The TryHackMe profile stays, and
  so does the LinkedIn profile: for now it carries the references, with
  the clients named, until the site can stand on its own.
- The current employer and the clients are anonymized, and colleagues
  are not named.
- The competence is kept apart from the employer's inventory of
  systems, to avoid security issues. Three questions decided each line:
  - does it describe today's systems or past ones;
  - can it already be seen from outside;
  - does it describe a security control.
- Two current projects are left out: one just started, and a training
  course still being defined.

### Language and title

**Chosen:** the site is in English; Italian is removed. The page title
is "Curriculum Vitae", in place of "Home Page"; the browser tab reads
"Luca Lorenzon - Curriculum Vitae".

**Consequences:** the card
[give the screen reader container an Italian voice](../backlog/give-the-screen-reader-container-an-italian-voice.md)
loses its premise (visitors expected to be mostly Italian), and the
screen reader's reference transcript will change. Both are carried by
the card
[update the screen reader check for an English site](../backlog/update-screen-reader-check-for-english-site.md).

### Length and structure

**Who reads the page:** someone deciding whether to engage the author.
In the first half minute they should understand that the author has the
skills asked for, in depth; years of field experience; interests beyond
one subject; and a way of working across technology and people, as a
product engineer rather than a software developer. The profile now says
"product engineer".

**Chosen:** each role has one line of description followed by its main
work, which replaced the role's bullet points and removed what they
repeated. About 1,800 words, accepted for this first increment.

**Rejected:** folding the details behind a click (`<details>`): it needs
HTML inside the Markdown, which the user wants to avoid. Kept as a
question for later.

### The look

**Chosen:** one centred column, at most 80 characters wide
(`long-text-column`), with rules for the elements inside it: sections
under an amber line (except the first, which sits right under the
header's line), roles in inverse video, "Main work" underlined, square
list markers, 32 pixels of space before the footer at the end of the
page, spacing in multiples of 8 pixels so the bitmap font stays sharp.

**Why inside one variant:** the HTML rendered from Markdown carries no
classes, and ADR-001 allows only named variants in the page. A
Tailwind 4 `@utility` accepts rules for nested elements, with the width
prefixes too (checked before the change); ADR-001 records this use.

**Temporary:** at the second article the concepts shared by every page
written in Markdown are abstracted; at the third, the look is tied to a
design system.

**Rejected:** Tailwind's typography plugin (its sizes are designed for
another font, and it brings Tailwind's own class names into the page).

### Tests

- The accessibility tree test compares the header and the footer
  element by element, as before, but for the content only the order of
  its section headings and the absence of a second level-1 heading: the
  full text would have to change with every edit of the curriculum.
- A new browser test checks the content on the same twelve devices as
  the header: no horizontal scroll, lines within 80 characters, section
  headings larger than the text, roles distinct, at least 32 pixels
  between the end of the content and the footer, no overlaps, no line
  above the first section.

## Results

- `8545db8`: card started.
- `0417794`: card [take the site's content from several repositories](../backlog/take-content-from-several-repositories.md) opened.
- `b11b743`: the site version of the curriculum, in `content/curriculum.md`.
- `2f1b629`: the page renders the curriculum, in English, titled "Curriculum Vitae".
- `9d57ea0`: the look of headings, lists and spacing, with the content layout test.
- `40b4cb8`: no line above the first section.
- `af6c52d`: space between the end of the content and the footer.

All checks pass: formatting, clippy, the Rust unit tests and the browser
tests. axe reports no violations. The screen reader check was not run
in this session (it needs Docker, run by the user).
