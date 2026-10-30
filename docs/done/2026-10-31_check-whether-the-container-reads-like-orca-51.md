# Check whether the container reads like Orca 51

- **Opened:** 2026-10-30, asked by the user when the card
  [logo name not read by Orca 51](../done/2026-10-30_logo-name-not-read-by-orca-51.md)
  was closed with its two open questions.
- **Status:** done on 2026-10-31. The container can be trusted: what it
  reported was Orca 51's own behaviour.

## Why

The screen reader container is the only check of what Orca actually
says. It mixes Firefox from Debian trixie with Orca 51 and the
accessibility libraries from trixie-backports, and Firefox logs warnings
about that mix. If the mix changed what Orca says, a transcript from the
container could report a problem a visitor would not have, or miss one
they would.

The doubt came from the old logo: Orca 51 in the container read a `<pre
role="img" aria-label="Lvk@73r">` as a code block and never spoke its
name, while Orca 50.3 on the user's desktop read the name. Replacing the
`<pre>` with a `<div>` fixed it, which proved the `<pre>` was the cause in
the container, but not whether the cause was Orca 51 itself or Orca 51
with the mixed libraries.

## Answers

### Is it Orca 51's behaviour?

**Yes.** Answered by reading Orca's source code (repository
`gitlab.gnome.org/GNOME/orca`, tags `50.3` and `51.1`), not by a second
container:

- Orca 51 added the presentation of code blocks (commit `612207210`,
  2026-03-28, "add support for article and code block"). Orca 50.3 has no
  such presentation, which is why it read the name.
- `is_code` in `src/orca/ax_utilities_role.py` treats every element whose
  tag is `pre` as code, with one exception for Thunderbird's quoted
  replies. The element's ARIA role is not looked at, so `role="img"` does
  not change the outcome.
- A code block is spoken as "start of code", its text, "end of code",
  with no name. An image exposes no text, so only "code start", "code
  end" was left: the container's transcript.
- The libraries take no part in that decision: Firefox only has to
  expose the `pre` tag, which it does.
- The same code is still on Orca's main branch on 2026-10-30.

**Rejected:** a reading with Firefox and Orca from the same Debian
release (a second container). The source answered the question without
it, and it would have needed a new image run by hand.

### Choice or bug, and is it reported upstream?

**Chosen:** no report, by the user. Presenting `<pre>` as code is a
choice of Orca; ignoring an explicit `role="img"` on it looks like an
oversight, and no Orca issue mentions it. But a `<pre>` was misleading for
the logo anyway: the logo is not preformatted text, so the site's fix
(the `<div>`) is the right one regardless of what Orca does.

**Rejected:** writing an issue for Orca's GitLab, with a minimal page and
the code reference. Not worth it for the site, which no longer has the
case.

## Results

- The container's transcripts can be trusted for what was observed. The
  only time it differed from the desktop, the difference was Orca 51's.
- Limit: this covers that one case. Firefox's warnings about the library
  mix were not explained; so far they have not changed any reading.
- Time spent: well under the half hour agreed for the source reading.
