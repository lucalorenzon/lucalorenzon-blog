# Check whether the container reads like Orca 51

- **Opened:** 2026-10-30, asked by the user when the card
  [logo name not read by Orca 51](../done/2026-10-30_logo-name-not-read-by-orca-51.md)
  was closed with its two open questions.
- **Status:** being worked on since 2026-10-31.

## Why

The screen reader container is the only check of what Orca actually
says. It mixes Firefox from Debian trixie with Orca 51 and the
accessibility libraries from trixie-backports, and Firefox logs warnings
about that mix. If the mix changes what Orca says, a transcript from the
container can report a problem a visitor would not have, or miss one they
would.

The first case may already have happened: Orca 51 in the container read
the old `<pre role="img">` logo as a code block and never spoke its name.
The logo was fixed by changing its markup, so the site no longer depends
on the answer, but the check does.

## Questions

- Is presenting a `<pre role="img">` as a code block, without its name,
  really Orca 51's behaviour? A run with Firefox and Orca from the same
  release (for example Debian testing), or Orca 51's release notes, would
  tell.
- If it is, is it a choice of Orca or a bug to report upstream?
