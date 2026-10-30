# Logo name not read by Orca 51

- **Opened:** 2026-10-30, asked by the user after an Orca 51 run of the
  card
  [run the screen reader check in a container](../done/2026-10-30_run-screen-reader-check-in-a-container.md).
- **Status:** done on 2026-10-30. The two questions about the container
  moved to the card
  [check whether the container reads like Orca 51](../backlog/check-whether-the-container-reads-like-orca-51.md).

## Why

On wide screens the logo was ASCII art in a `<pre role="img"
aria-label="Lvk@73r">` (`src/home_page.rs`). Orca 50, on the user's
desktop, read it as "Lvk @ 73r", "image". Orca 51.1 in the screen reader
container said only "code start", "code end": a person using a recent
Orca would not hear the site's name in the header.

Firefox exposed the logo correctly, as an image named "Lvk@73r"; Orca 51
presented it as a code block, most likely because the element was a
`<pre>`, and a code block's name is not spoken.

## Answers

### What comes first: the site or the check?

The card asked three questions: whether the behaviour is really Orca 51's
or comes from the container (Firefox from Debian trixie, accessibility
libraries from trixie-backports); if it is Orca's, whether it is a choice
or a bug; and what markup lets every recent screen reader name the logo.

**Chosen:** make the logo's name readable first, by the user. It is what
a visitor hears. A reading in the container is enough to check the fix:
if Orca 51 speaks the name there, even with the mixed libraries, the
markup works.

**Rejected:** first finding out whether the container is reliable. It
matters for the check, not for visitors, and it does not block the fix.
Its two questions are now a card of their own (see "Status").

### Which markup?

**Chosen:** the same element without the `<pre>` tag: a `<div
role="img" aria-label="Lvk@73r">` holding the ASCII art, kept aligned by
CSS `white-space: pre` in the `art-logo-left` style. The smallest change:
the logo stays an image with a name for every screen reader, the look and
the accessibility tree do not change, and the existing tests stay valid.

**Rejected:** reading the name as text and hiding the drawing
(`aria-hidden`), with the narrow-screen text logo kept readable on wide
screens too. The user called it a workaround: it changes what the logo is
for a screen reader instead of removing the cause.

## Results

- A unit test, `logo_is_not_a_pre_element`, failed before the change and
  passes after it; all other unit tests and the browser tests (layout and
  accessibility tree at every width) passed before and after.
- The container check (Orca 51.1, Firefox 153.6.0esr) now reads
  "Lvk@73r", "image" where it said "code start", "code end". The reference
  transcript was updated: `docs/accessibility/screen-reader-checks.md`,
  run of 2026-10-30 23:13.
- The log sample used by the screen reader script's own tests
  (`scripts/screen-reader/fixtures/orca51-page-read.log`) still holds the
  old reading. It stays as it is: it is a real Orca log that tests how the
  script reads Orca's output, not a description of the site.
