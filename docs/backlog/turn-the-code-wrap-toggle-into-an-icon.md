# Turn the code wrap toggle into an icon

- **Opened:** 2026-11-14, by the user, after seeing the code block page
  of the style guide
  ([card](../wip/show-the-code-block-in-the-style-guide.md)); in Next.
- **Changed:** 2026-11-14, by the user: done before the style guide is
  published, in a session of its own; in Now.
- **Status:** noticed, not started.

## Why

A code block whose longest line does not fit shows a checkbox with the
word "wrap" above the code. The user wants an icon in its place.

## Known from the start

- **How the toggle works today** (`code_component` in
  `src/markdown.rs`, `code-wrap-toggle` and `wrap-shown` in
  `style/input.css`): a `<label>` holding a checkbox, named for screen
  readers "Wrap long lines of code", and the visible word "wrap". It
  shows only when the longest line does not fit (a container query);
  checking it wraps the lines through CSS alone. The site runs no
  JavaScript, so the toggle keeps working without it.
- **What an icon must keep:**
  - a name for screen readers, since an icon has no text of its own;
  - the state, checked or not, announced and visible;
  - the focus outline, already drawn in the line accent;
  - the contrast of a graphic, at least 3:1 against both backgrounds
    (WCAG 1.4.11), which the line accent and the code accent meet;
  - a target large enough for a finger (WCAG 2.5.8: at least 24 by 24
    CSS pixels, or enough space around it).
- **The tests that know the toggle** (`end2end/tests/code-blocks.spec.ts`,
  rows in `end2end/README.md`): they find it as the block's `label` and
  `input`, reach it with Tab and switch it with Space, and check that
  copying a block does not copy the word "wrap". With an icon the last
  check changes meaning, and the selectors may change.
- **Where it is tested:** the style guide's code block page, since
  the card above; the icon is a change of the component, so it shows
  there first.

## Questions

- Which icon, and drawn how: a character of a font, an SVG in the
  page, a file.
- Whether the checkbox stays, hidden behind the icon, so the toggle
  keeps working without JavaScript.
- Whether a word stays next to the icon, or a tooltip, for the readers
  who do not recognise it.
- Whether it comes before the style guide is published, as part of the
  code block's page, or after. Answered by the user on 2026-11-14:
  **before**, in a session of its own.
