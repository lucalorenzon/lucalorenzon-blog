# Add a copy button to code blocks

- **Opened:** 2026-11-15, by the user, after choosing the wrap toggle's
  icon in the card
  [turn the code wrap toggle into an icon](../done/2026-11-15_turn-the-code-wrap-toggle-into-an-icon.md);
  in Later.
- **Status:** noticed, not started.

## Why

A reader who wants a block's code today selects it by hand; the
browser tests already check that a selection gives the code and
nothing else. A button that copies the whole block in one gesture
spares the selection, which is awkward on a phone and on a block that
scrolls sideways. It would be the code block's second icon, next to
the wrap toggle.

The user asked to weigh WebAssembly, and preferably `use_clipboard`
from [leptos-use](https://leptos-use.rs).

## Known from the start

- **Copying needs a script.** The stylesheet can show and hide, but it
  cannot write to the clipboard: only the browser's Clipboard API
  (`navigator.clipboard.writeText`) can, and only from a script. The
  site sends no script today, so this would be its first.
- **`use_clipboard` is WebAssembly, not an alternative to it.**
  leptos-use 0.20.0 (MIT or Apache 2.0) targets Leptos 0.8; its
  `use_clipboard` feature, which can be enabled alone, calls the
  Clipboard API through `web-sys` and `wasm-bindgen`. It runs only in
  the browser, so the button would be a Leptos island: a component
  compiled to WebAssembly and brought to life on the static page. The
  choice is then between an island using `use_clipboard` and a few
  lines of plain JavaScript, with no WebAssembly.
- **Whether islands work on this site is not known yet.** The site is
  rendered at build time with no server. The card
  [measure what a Leptos island weighs](measure-weight-of-a-leptos-island.md)
  waits for exactly this first island, and its spike asks both what an
  island weighs and whether islands work without a server.
- **The Clipboard API asks for a secure page and a gesture**: HTTPS,
  which GitHub Pages gives, and a click or a key press.
- **Without the script the button must not show**, or it would do
  nothing; the selection keeps working as today.
- **What a button must keep**, as the wrap toggle does: a name for
  screen readers, the focus outline, the contrast of a graphic, a
  target of at least 24 by 24 CSS pixels; and a way to say the copy
  happened, also to a screen reader.
- **Where it is tested:** the style guide's code block page, which
  then explains its icon in the section agreed for the wrap toggle's.

## Questions

- An island with `use_clipboard`, or plain JavaScript: what each
  weighs, and what each asks of the build and of the tests.
- Whether it waits for the island spike, or answers it.
- Which icon, from the same set as the wrap toggle's, and how the
  button says the copy happened.
- Where it sits in the block, next to the wrap toggle, which shows
  only when a line does not fit, while a copy button would show on
  every block.
