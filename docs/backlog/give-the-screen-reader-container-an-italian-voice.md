# Give the screen reader container an Italian voice

- **Opened:** 2026-10-30, asked by the user after listening to the first
  recording of the card
  [run the screen reader check in a container](../done/2026-10-30_run-screen-reader-check-in-a-container.md).
- **Status:** noticed, not started.

## Why

The container's recording (`target/screen-reader/reading.wav`) is there
to judge how a name or a text sounds. In the first recording, Orca's own
words are mispronounced: "leaving banner" is read with the Italian voice,
"leaving" spoken as if it were an Italian word.

Why it happens: the container has no Italian locale, so Orca speaks its
own words in English; speech-dispatcher keeps the last language it was
given, which is Italian after the page's Italian text, so English words
come out with the Italian voice. The transcript does not show it: it
records the words and the language Orca asked for, not how they sound.

It does not depend on the site: a visitor with Orca in Italian would hear
"uscita da banner" with the Italian voice. But it makes the recording a
poor guide to what the site's visitors hear, who are expected to be
mostly Italian.

## Idea to try

Run the container with an Italian locale, so Orca speaks its own words in
Italian, as for an Italian visitor.

## Questions

- Does an Italian locale make Orca 51 speak its words in Italian, and its
  default voice Italian? Which packages does that add to the image?
- The transcript would change (Orca's words, and possibly the language of
  the default voice, recorded today as `system`): a new reference, to be
  accepted as an intended change.
- Should the check read the page once per language a visitor may use, or
  is Italian enough while the site is Italian?
- Does the logo's name ("Lvk@73r") sound different with an Italian
  default voice? That links to the card
  [logo name not read by Orca 51](logo-name-not-read-by-orca-51.md).
