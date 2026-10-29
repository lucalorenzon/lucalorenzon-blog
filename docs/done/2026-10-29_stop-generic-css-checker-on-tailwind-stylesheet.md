# Stop the generic CSS checker on the Tailwind stylesheet

- **Opened:** 2026-10-29, raised by the user while closing the card
  [use Tailwind's canonical class for the 32px line height](2026-10-29_use-tailwind-canonical-line-height.md).
- **Status:** done on 2026-10-29, in commit `26b4743`.

## Why it was opened

The editor (Zed) showed 2 errors and 14 warnings on `style/input.css`,
although the site builds and its stylesheet is right. They came from
the generic CSS checker (`vscode-css-language-server`), which does not
know Tailwind 4's syntax:

- line 3, `@import "tailwindcss" source(none);`: "semi-colon expected"
  and "{ expected";
- "Unknown at rule" for `@source` (line 4), `@theme` (line 12) and the
  12 `@utility` blocks.

They were reproduced with the same checker the editor installs, and were
identical before and after the line-height change that made them
noticed.

## Answers

### How does the editor stop reporting valid Tailwind as errors?

**CSS files are checked by Tailwind's own CSS server instead of the
generic one.** Zed's
[Tailwind documentation](https://zed.dev/docs/languages/tailwindcss)
presents `tailwindcss-intellisense-css` as an alternative to the generic
checker, with this setting:

```json
"languages": {
  "CSS": {
    "language_servers": [
      "tailwindcss-intellisense-css",
      "!vscode-css-language-server",
      "..."
    ]
  }
}
```

`!` turns the generic checker off; `"..."` keeps the other default
servers. Tailwind's server knows Tailwind 4's syntax, `@utility` and
`@source` included, although the documentation names only `@apply`,
`@layer` and `@theme`: the user checked it in the editor.

Rejected: telling the generic checker to ignore unknown at-rules. That
would have silenced the 14 warnings but not the 2 errors on
`source(none)`, which is Tailwind syntax the standard CSS grammar cannot
parse.

### Where does the setting live?

**In `.zed/settings.json`, in the repository (chosen by the user).** The
reason for the setting is Tailwind 4, a property of this project, not of
the person editing it; a project file keeps the choice versioned with
the code it serves.

Rejected: the user's own Zed settings, which would have turned the
generic checker off in every project, including those with plain CSS,
where it is useful. The cost accepted is an editor-specific file in
git, while other editor files (such as `.idea`) stay out of it.

## Results

- The user confirmed that Zed shows no errors and no warnings on
  `style/input.css` any more.
