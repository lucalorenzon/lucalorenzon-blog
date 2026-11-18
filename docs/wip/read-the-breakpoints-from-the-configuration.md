# Read the breakpoints from the configuration

- **Created:** 2026-11-18
- **Status:** In work

## Why

The user opened this card on 2026-11-18, while cutting the last page
of the style guide before it is published, the breakpoints (card
[build a design system](build-a-design-system.md)). The user wants
that page to be both the explanation of how the site adapts to the
screen width and the only place where the breakpoints are set: a new
breakpoint is added there, and nowhere else.

Today the breakpoints are written in three places that know nothing
of each other: the stylesheet, the browser tests and the Rust checks
of the classes. Changing one of them means finding the others by
hand, and forgetting one breaks nothing visible until a test or a
page goes wrong at some width.

## What is known

- **The site's breakpoints are two**, set by hand in `style/input.css`
  (`@theme`), after clearing Tailwind's own with
  `--breakpoint-*: initial`: `md` at 48rem (768 pixels) and `xl` at
  80rem (1280 pixels). Checked on 2026-11-18.
- **What changes at each one** stays in the components, as classes
  with the breakpoint's prefix (for example `md:title-below-line-large`
  in the header, `src/page_frame.rs`). At 768 pixels: the written logo
  becomes the logo drawn in characters, the page title grows from 24
  to 32 pixels, the text of the column from 18 to 20 pixels and the
  column widens, the footer's text grows. At 1280 pixels: the header
  becomes one row, with the title beside the logo.
- **Two thresholds are not breakpoints of the site:** the code block's
  wrap toggle and the table's record viewer change form with the
  width of their own container, at a threshold the build estimates for
  each block (`src/markdown.rs`). This card does not touch them.
- **The browser tests write the widths by hand.** 767, 768 and 1280
  appear in `end2end/tests/support.ts`, `header-layout.spec.ts`,
  `code-blocks.spec.ts`, `home.spec.ts` and `font-roles.spec.ts`.
  Some are the two sides of a breakpoint (767 and 768 in "column
  widths", the bands of the header); others are real devices that
  happen to have that width (iPad Mini at 768, a laptop at 1280).
  320, 393 and 412 are devices only (`end2end/README.md`, "Widths").
- **The Rust checks of ADR-001** accept a class with the prefix `md:`
  or `xl:` written in the code, in three modules: `src/home_page.rs`,
  `src/curriculum_page.rs` and `src/style_guide.rs`.
- **How the fonts did it** (card
  [link the fonts to the build](../done/2026-11-16_link-the-fonts-to-the-build.md)):
  a text file in `config/`, one `key=value` per line, read by the
  build, which writes a stylesheet in `target/generated/` with its own
  `@theme` block, imported by `style/input.css`. Tailwind accepts a
  `@theme` block in an imported file; the fonts already rely on it.

## Questions to answer

The ones known when the card was written; there may be more.

- **Where the values live, and who reads them.**

  **Answered by the user on 2026-11-18.** In one place, read by the
  stylesheet, the style guide's page and the tests; what changes at
  each breakpoint stays in the components. Rejected: making the page
  also the place of the changes, since the changes are the
  components' own classes.
- **In which form.**

  **Answered by the user on 2026-11-18.** A text file in `config/`,
  one line `name=width`, as `config/fonts.txt`, because the tests too
  must read it, and `config/` is the only place both the build and
  the tests read. Rejected: Rust, as the colours, which the tests
  cannot read; and doing first the card on a structured configuration
  file, which would make the style guide wait for a choice of format
  not made yet. That card gets a third setting to move.
- **Whether the page comes with this card.**

  **Answered by the user on 2026-11-18.** No: two cards, the values
  first, then the page
  ([show the breakpoints in the style guide](../backlog/show-the-breakpoints-in-the-style-guide.md)).
- **In which unit the width is written.** Tailwind takes rem (48rem),
  the tests set the window in pixels. A breakpoint in rem moves when
  the reader changes the browser's default text size, which keeps the
  layout in step with a larger text; the tests run at the default
  size, where 1rem is 16 pixels.
- **Which widths of the tests come from the file.** The two sides of
  each breakpoint, surely; the devices that happen to share a width
  with one, perhaps not, since a device does not change when a
  breakpoint moves.
- **What the file may contain**, and what stops the build: a missing
  or repeated name, a width that is not a number, a name Tailwind
  would not accept as a prefix.

## Dependencies

None known.

## Acceptance tests

To be agreed with the user before development starts.

## What it leaves

- The style guide's page on the breakpoints:
  [show the breakpoints in the style guide](../backlog/show-the-breakpoints-in-the-style-guide.md).

## References

- [Build a design system](build-a-design-system.md): the card this
  one serves; its list of pages before publishing ends with the
  breakpoints.
- [Show the breakpoints in the style guide](../backlog/show-the-breakpoints-in-the-style-guide.md):
  the page that reads the values this card puts in `config/`.
- [Link the fonts to the build](../done/2026-11-16_link-the-fonts-to-the-build.md):
  the same path, from a file in `config/` to a stylesheet written by
  the build.
- [Use a structured configuration file](../backlog/use-a-structured-configuration-file.md):
  the breakpoints become its third setting to move.
