# Drop the wrap toggle's leftover font

- **Opened:** 2026-11-15, by the agent, while closing the card
  [turn the code wrap toggle into an icon](turn-the-code-wrap-toggle-into-an-icon.md);
  done in the same session, at the user's request.
- **Status:** done, 2026-11-15. Not published yet; nothing a reader
  sees changes.

## Why

`code-wrap-toggle` in `style/input.css` still applies `font-sans`,
which gave the word "wrap" the text's font. The word is gone: the
label holds only the checkbox drawn as an icon, so the font sets
nothing a reader sees, and a rule that serves nothing misleads whoever
reads the stylesheet next.

## Answer

**`font-sans` is removed from `code-wrap-toggle`.** The label has no
text, so no line box, and its size comes from the checkbox and its
padding alone.

## Results

Both acceptance tests passed on 2026-11-15:

1. `code-wrap-toggle` no longer applies `font-sans`.
2. The label measures 28 by 28 CSS pixels in the built code page, and
   every check of the pipeline passes: 325 unit tests, 267 browser
   tests passed and 1 skipped, the one that runs on the live site
   only.
