# Drop the wrap toggle's leftover font

- **Opened:** 2026-11-15, by the agent, while closing the card
  [turn the code wrap toggle into an icon](../done/2026-11-15_turn-the-code-wrap-toggle-into-an-icon.md);
  done in the same session, at the user's request.
- **Status:** in progress since 2026-11-15.

## Why

`code-wrap-toggle` in `style/input.css` still applies `font-sans`,
which gave the word "wrap" the text's font. The word is gone: the
label holds only the checkbox drawn as an icon, so the font sets
nothing a reader sees, and a rule that serves nothing misleads whoever
reads the stylesheet next.

## Acceptance tests

1. `code-wrap-toggle` no longer applies `font-sans`.
2. The label is still 28 by 28 CSS pixels, and every check of the
   pipeline passes, the browser tests on the toggle included.
