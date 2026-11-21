# Declare DOM.Iterable for the editor's TypeScript

- **Opened:** 2026-11-08, by the user, who saw five errors in the editor
  on `end2end/tests/table-forms.spec.ts` (three) and
  `end2end/tests/tables.spec.ts` (two), while the build's type check
  passed.
- **Status:** done, 2026-11-08.

## Why

The five errors are the five places that spread a list of the page's
elements (`[...document.querySelectorAll(...)]`, `[...getClientRects()]`),
there since 2026-11-05. The editor (Zed) checks the tests with the
TypeScript bundled in its language server, 5.9.3; the project uses
TypeScript 7.0.2, the native compiler, which the editor cannot use in its
place. From version 6 TypeScript counts those lists as iterable in the
`DOM` library alone; 5.9 needs the `DOM.Iterable` library as well, which
`end2end/tsconfig.json` does not declare.

## Answers

### How are the errors removed?

`DOM.Iterable` is added to the libraries of `end2end/tsconfig.json`,
with a comment saying it is there for the editor's older TypeScript.

**Rejected:** making the editor use the project's TypeScript 7.0.2: the
native compiler does not ship the server the editor's language server
loads, so the editor cannot take it in place of its own.

## Results

- With the editor's TypeScript 5.9.3 on the project's configuration:
  five errors before, none after. With TypeScript 7.0.2
  (`npm run typecheck`): none before or after.
- The table tests pass unchanged (52).
- While the two versions differ, an error of this kind shows only in
  the editor and not in the build.

Commit: `8b2262d`.
