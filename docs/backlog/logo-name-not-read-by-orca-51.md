# Logo name not read by Orca 51

- **Opened:** 2026-10-30, asked by the user after an Orca 51 run of the
  card
  [run the screen reader check in a container](../wip/run-screen-reader-check-in-a-container.md).
- **Status:** noticed, not started.

## Why

On wide screens the logo is ASCII art in a `<pre role="img"
aria-label="Lvk@73r">` (`src/home_page.rs`). Orca 50, on the user's
desktop, reads it as "Lvk @ 73r", "image". Orca 51.1 (Debian
trixie-backports, in the trial container, with Firefox 153.6.0esr)
says only "code start", "code end": the name is never spoken. A person
using a recent Orca would not hear the site's name in the header.

What the debug file shows: Firefox exposes the logo correctly, as an
image named "Lvk@73r"; Orca 51 presents that image as a code block
(its braille generator calls `_generate_code_block` for it), most
likely because the element is a `<pre>`, and a code block's name is not
spoken.

## Questions

- Is it really Orca 51's behaviour? The container mixes Firefox from
  trixie with accessibility libraries from trixie-backports, and Firefox
  logs warnings about that mix. A run with Firefox and Orca from the
  same release (for example Debian testing), or Orca 51's release notes,
  would tell.
- If it is, is it a choice of Orca or a bug to report upstream?
- What would the markup need so every recent screen reader names the
  logo, while the ASCII art stays hidden from it? The accessibility tree
  test and the narrow-screen text logo must keep working.
