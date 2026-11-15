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
- Which icon, and what tells its two states apart. Answered by the
  user on 2026-11-15, after two trials on the style guide's code page:
  **Bootstrap Icons' `BsTextWrap` in both states; when the lines wrap,
  the label takes a filled background in the code accent and the icon
  turns to the code surface's colour.** The icon names the function,
  the fill tells whether it is on, as a pressed button in a toolbar:
  the same meaning as the checkbox underneath, so what the page shows
  and what a screen reader announces ("Wrap long lines of code",
  checked or not) agree. Rejected, from `icondata` 0.7.0's wrap icons:
  - two drawings, one per state (Bootstrap's `BsArrowBarRight` with
    `BsTextWrap`, Tabler's `TextWrapDisabled` with `TextWrap`, Material's
    `WrapDisabled` with `Wrap`): a drawing can be read as the state or
    as the action. The first trial drew the state; the user wanted the
    action, and the second trial swapped them; but an icon showing the
    action contradicts the checkbox's announced state. Tabler's two
    drawings also differ too little at 24 pixels;
  - the same icon slashed when off: dense over thin lines at 24 pixels;
  - a frame instead of a fill when on: a weaker signal than the fill.
- Whether a word stays next to the icon, or a tooltip, for the readers
  who do not recognise it. Answered by the user on 2026-11-15: **the
  browser's own tooltip, a `title` on the label**, with no word: the
  sign is known to whoever has used an editor, and is learnt by using
  it. The tooltip helps who does not recognise it; the guidelines do
  not require it, since the checkbox already has its accessible name.
  On the label, not on the checkbox, so a screen reader does not read
  the name twice. Rejected: a word next to the icon, the text just
  removed; a tooltip drawn by the stylesheet, which over the code
  would hide lines and, without JavaScript, could not be closed with
  Esc as WCAG 1.4.13 asks (the browser's own tooltip is exempt).
- Where the style guide explains the icon. Answered by the user on
  2026-11-15: **a new section of the code block page**
  (`content/style-guide/code.md`), on the toggle's icon and its two
  states; the agent writes a placeholder saying what kind of text goes
  there, and the user writes the text, as for the page's other texts.
  Rejected: a page of its own for the icons, since probably only one
  more icon will come, for copying a block to the clipboard, and it
  belongs to the same component.

## Found in the trials

- **The mask hides the checkbox's own focus outline**, since a mask
  clips everything the element draws. In the trial the outline moved
  to the label, where the scroller's edge cuts it at the top and the
  left. It must be solved, and a test must check it is visible.
- **The wrapping, the toggle's showing only when a line overflows, and
  the code accent's contrast hold** in both schemes, at 375 and 1280
  pixels.
- **The filled state needs a pair of colours not allowed today**: the
  code surface on the code accent. The ratio is the same both ways
  (7.16 dark, 7.12 light), but `ALLOWED_PAIRS` in
  `src/style_guide/colours.rs` forbids every pair it does not list.

## Slices

Agreed with the user on 2026-11-15; one commit each, in this order.

1. **Allow the code surface on the code accent, for graphics**: the
   colours module, its tests, the colours page.
2. **The icon**: the build writes the mask from `icondata`; the
   checkbox becomes Bootstrap's icon, filled when on, with the tooltip
   on the label and the focus outline whole; Bootstrap Icons' licence
   checked at source, with its attribution; the browser tests and
   their README rows.
3. **The section on the icon** in the code block page, with a
   placeholder the user then writes.

## Acceptance tests

Agreed with the user on 2026-11-15. The card is done when all hold.

Slice 1, the pair of colours:

1. The code surface on the code accent is an allowed pair for
   graphics, and the build checks it reaches 3:1 in both schemes.
2. The style guide's colours page shows the new pair with the others.

Slice 2, the icon:

3. On the code block page, the block with the long line shows the
   toggle as the icon, with no word; the block that fits shows none.
4. Off, the icon is in the code accent with no fill; on, the label is
   filled in the code accent and the icon takes the code surface's
   colour; in both schemes.
5. The target is at least 24 by 24 CSS pixels.
6. Tab reaches the toggle and Space switches it; its focus outline is
   visible whole, not cut by the block's edge.
7. To assistive technologies the checkbox stays a checkbox, named
   "Wrap long lines of code", its state following the check; the name
   does not come again as a description.
8. The label carries the browser's tooltip "Wrap long lines of code".
9. Copying a block, wrapped or not, gives the code and nothing else.
10. The build writes the icon's drawing from `icondata`, at a pinned
    version and with Bootstrap's set only; the stylesheet holds no
    drawing written by hand.
11. Bootstrap Icons' licence is read at source and recorded with its
    attribution.
12. An article with a long line shows the same icon, with no change
    outside the component.
13. The screen reader check reads the toggle's name once (run by the
    user, since it uses Docker).
14. The rows of the browser tests' README are up to date, and every
    check of the pipeline passes.

Slice 3, the section:

15. The code block page has a section on the toggle's icon and its two
    states, with the text written by the user.
