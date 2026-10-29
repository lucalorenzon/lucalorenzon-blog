# Stop the generic CSS checker on the Tailwind stylesheet

- **Opened:** 2026-10-29, raised by the user while closing the card
  [use Tailwind's canonical class for the 32px line height](../done/2026-10-29_use-tailwind-canonical-line-height.md).
- **Status:** noticed, not started.

## Why

The editor (Zed) shows 2 errors and 14 warnings on `style/input.css`,
although the site builds and its stylesheet is right. They come from
the generic CSS checker (`vscode-css-language-server`), which does not
know Tailwind 4's syntax:

- line 3, `@import "tailwindcss" source(none);`: "semi-colon expected"
  and "{ expected";
- "Unknown at rule" for `@source` (line 4), `@theme` (line 12) and the
  12 `@utility` blocks.

Reproduced on 2026-10-29 with the same checker the editor installs.

## Questions

- Where the setting lives. Zed's documentation on Tailwind suggests
  replacing the generic checker with `tailwindcss-intellisense-css` for
  CSS files (`"language_servers": ["tailwindcss-intellisense-css",
  "!vscode-css-language-server", "..."]`). It can go in a project file
  `.zed/settings.json`, valid for this repository only, or in the user's
  own Zed settings, valid for every project. The agent prefers the
  project file, so the choice stays with the project that needs it; but
  until now editor files (such as `.idea`) are kept out of git.
- Whether Tailwind's checker also accepts `@utility` and `@source`:
  Zed's page mentions only `@apply`, `@layer` and `@theme`. To be tried
  in the editor.
