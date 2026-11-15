# Turn the code wrap toggle into an icon

- **Opened:** 2026-11-14, by the user, after seeing the code block page
  of the style guide
  ([card](../done/2026-11-15_show-the-code-block-in-the-style-guide.md)); in Next.
- **Changed:** 2026-11-14, by the user: done before the style guide is
  published, in a session of its own; in Now.
- **Status:** in progress since 2026-11-15.

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

- Whether it comes before the style guide is published, as part of the
  code block's page, or after. Answered by the user on 2026-11-14:
  **before**, in a session of its own.
- Whether the checkbox stays, hidden behind the icon, so the toggle
  keeps working without JavaScript. Answered by the user on
  2026-11-15: **the checkbox stays, and the stylesheet draws it as an
  icon with two states.** It keeps its name for screen readers, its
  place in the Tab order and its state, and the site still needs no
  JavaScript.
- How the stylesheet draws it. Answered by the user on 2026-11-15,
  from the agent's explanation: **the checkbox loses its native look
  and the icon is its mask**, so the icon takes the site's colours,
  follows the light and dark schemes, and its contrast is that of a
  pair the build already checks.
- Where the icon's drawing comes from. Answered by the user on
  2026-11-15: **the build writes it**, as it writes the colours
  (`target/generated/colours.css`), reading it from `icondata`, the
  data crate of `leptos_icons`, limited to the one set the icon comes
  from. Rejected: `leptos_icons`' components, which write an `<svg>`
  into the page, not into the stylesheet; and the drawing copied by
  hand into `style/input.css`, which needs no dependency but leaves
  the drawing's origin and version to a comment. The set's own licence
  applies, not the crate's, since the drawing is served with the page:
  it is recorded with its attribution, as the fonts' are.
- Which icon, and what tells its two states apart.
- Whether a word stays next to the icon, or a tooltip, for the readers
  who do not recognise it.
