# Turn the code wrap toggle into an icon

- **Opened:** 2026-11-14, by the user, after seeing the code block page
  of the style guide
  ([card](../done/2026-11-15_show-the-code-block-in-the-style-guide.md)); in Next.
- **Changed:** 2026-11-14, by the user: done before the style guide is
  published, in a session of its own; in Now.
- **Status:** done, 2026-11-15. Not published yet: the articles get
  the icon with the next version, the style guide when it is complete.

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

## Answers

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
- What the colours page's sample of the new pair shows, since the code
  surface is a colour for graphics there and the icon does not exist
  before the second slice. Answered by the user on 2026-11-15: **a box
  filled in the code accent with icons in it**, so the sample shows
  graphics, not text. The user first wanted several icons (the wrap
  toggle's, the future copy button's, light and dark, a document, a
  folder); the agent pointed out that choosing them is a decision of
  its own, that two of them have no use on the site yet, and that the
  sample must draw them the way the toggle will. **Only the wrap
  toggle's icon, drawn by the build from `icondata`**: the others come
  with the cards that need them, since choosing them now would be
  premature. So the drawing, the dependency and the licence move from
  the second slice to the first.

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

## Found while building the second slice

- **In Windows' forced colours mode the icon disappeared**, in both
  states (checked by the agent on 2026-11-15, emulating the mode in
  Playwright). The mode replaces the site's colours with a small
  palette the user chose; the icon is a background cut by a mask, so
  its background, and the label's fill that shows the state, took the
  page's own background. The native checkbox, drawn by the system,
  was visible: the icon was a step back for exactly these readers.
  Answered by the user on 2026-11-15: **in that mode only
  (`@media (forced-colors: active)`), the toggle takes the user's
  system colours, kept by `forced-color-adjust: none`**: off, the icon
  in `CanvasText`, with no fill; on, the label filled in `Highlight`
  and the icon in `HighlightText`, the colours the user's theme gives
  to what is selected; the focus outline as the browser recolours it.
  The reader asked to see their own colours, and the toggle honours
  it. Rejected: `ButtonText` for the icon when off, the colour of
  buttons, which in some themes barely differs from the text.

## How it was done

Three slices, one commit each, agreed with the user on 2026-11-15 and
cut again the same day, so the colours page could show the new pair
with the real icon:

1. **The pair of colours and the icon's drawing** (`323422b`): the code
   surface on the code accent allowed for graphics, with its sample on
   the colours page; the build writes the drawing from `icondata` 0.7.0
   (Bootstrap Icons 1.13.1 only) into `target/generated/icons.css`, as
   a mask utility, `icon-text-wrap`, that sets the shape and leaves the
   colour to the element's background; Bootstrap Icons' MIT licence,
   read at source, with its attribution in `assets/icons/`.
2. **The toggle as the icon** (`2e5de5e`): the checkbox without its
   native look, 20 pixels, masked by the icon, in a label of 28 by 28
   pixels; the focus outline is the label's, for keyboard focus only,
   and a 4 pixel margin around the label, as wide as the outline and
   its offset together, keeps the scroller's edge from cutting it. An
   outline drawn inside the label was set aside by the sub-agent: when
   on, it would sit on the code accent's fill, a pair the build does
   not check. The forced colours styles came in the same slice.
3. **The section on the icon** (`c4721ef`), with the user's text.

## Results

Every acceptance test, agreed with the user on 2026-11-15 and
renumbered when the slices were cut again, passed on 2026-11-15, but
one, moved to another card.

Slice 1, the pair of colours and the icon's drawing:

1. The code surface on the code accent is an allowed pair for
   graphics, and the build checks it reaches 3:1 in both schemes
   (7.12 light, 7.16 dark): unit tests of the colours module.
2. The colours page shows the wrap icon in the code surface's colour
   on a box filled in the code accent, in both schemes: unit test of
   the sample, and screenshots checked by the user.
3. The build writes the drawing from `icondata` at a pinned version,
   with Bootstrap's set only; a unit test fails if the stylesheet holds
   a drawing written by hand.
4. Bootstrap Icons' licence was read at source and is served with its
   attribution in `assets/icons/`.

Slice 2, the toggle as the icon; browser tests on the style guide's
code page, listed in `end2end/README.md`:

5. The block with the long line shows the icon with no word, the block
   that fits shows none.
6. Off, the icon is in the code accent with no fill; on, the label is
   filled in the code accent and the icon in the code surface's colour;
   in both schemes.
7. The target is 28 by 28 CSS pixels.
8. Tab reaches the toggle and Space switches it; the focus outline lies
   whole inside the scroller and the page, at 320 and 1280 pixels.
9. The checkbox is a checkbox named "Wrap long lines of code", its
   state following the check, with no accessible description.
10. The label carries the tooltip "Wrap long lines of code", the
    checkbox none.
11. Copying a block, wrapped or not, gives the code and nothing else.
12. An article with a long line shows the same icon, with no change
    outside the component: screenshots of the After Save Event
    article, off and on.
13. Moved on 2026-11-15 to the card
    [update the screen reader check for an English site](../backlog/update-screen-reader-check-for-english-site.md):
    the check opens the home page only, which has no code block, so it
    cannot read the toggle until it reads more pages. The browser test
    of 9 already checks, in Chromium's accessibility tree, that the
    name comes once and with no description; whether Orca in Firefox
    reads it so is left to that card.
14. In forced colours mode, emulated, the icon shows in both states in
    the user's system colours: off in `CanvasText`, on in
    `HighlightText` on a label filled in `Highlight`; the focus outline
    in `CanvasText`.
15. The rows of the browser tests' README are up to date, and every
    check of the pipeline passes: 325 unit tests, 267 browser tests
    passed and 1 skipped, the one that runs on the live site only.

Slice 3, the section:

16. The code block page has a section on the toggle's icon, with the
    text written by the user.
