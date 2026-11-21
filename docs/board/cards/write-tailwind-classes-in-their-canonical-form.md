# Write Tailwind classes in their canonical form

- **Opened:** 2026-11-15, by the user, after Tailwind's CSS server in
  the editor flagged classes of `style/input.css` that have a shorter,
  standard name; to be done in the same session, so it is not
  forgotten.
- **Status:** done, 2026-11-15. Not published yet: it changes the
  stylesheet's source only, and what the browser draws stays the same.

## Why

Tailwind's CSS server, which checks `style/input.css` in the editor
(`.zed/settings.json`), suggests a canonical name for a class written
with an arbitrary value that equals a value of Tailwind's own scale.
The suggestions stay on screen as warnings, and a warning that is
always there hides the one that matters. The canonical name also says
that the value is one of the scale's, not a value chosen for one place.

## What the server flagged

Five classes, on 2026-11-15:

- `size-[24px]`, in `sample-icon`: written by the first slice of
  [turn the code wrap toggle into an icon](turn-the-code-wrap-toggle-into-an-icon.md)
  and corrected there to `size-6` before its commit; not part of this
  card.
- `leading-[1.5]`, three times, in the headings' rules of the header
  (lines 135, 139, 143 on that day): `leading-normal` is 1.5 in
  Tailwind 4.
- `rounded-[8px]`, once (line 422 on that day): `rounded-lg` is
  0.5 rem, and the page fixes the root's font size at 16 pixels
  (`html, body` in `style/input.css`), so it is 8 pixels.
- `text-[18px]` inside `@utility footer-text-large`, and `ml-[0.5em]`
  inside `@utility breadcrumb-separator`: the server suggests replacing
  each with the very utility that contains it, which would make the
  utility call itself. A false alarm; it cannot be followed.

## Answers

- **The three classes with a canonical name take it**: `leading-normal`
  in the three headings' rules, `rounded-lg` in the list of articles.
- **The two false alarms: the two utilities are written in plain
  CSS** (`font-size: 18px;`, `margin-left: 0.5em;`), decided by the
  user on 2026-11-15, after asking why they appear. They hold no class
  the server can compare, and the browser gets the same CSS.
  Why they appear: the server looks for a name that gives the same CSS
  as a class with an arbitrary value; Tailwind's scale has none for
  these two (`text-lg` also sets the line height, and the scale has no
  `em`), so the only name left is the utility that holds the class
  alone. Rejected:
  - turning off `tailwindCSS.lint.suggestCanonicalClasses` (its
    README, read at source): it holds for the whole file, so it would
    also hide the useful suggestions, such as the three of this card;
  - leaving them on screen: two warnings always there hide the one
    that matters.

  [ADR-001](../../adr/ADR-001-name-styles-by-visual-variant.md) rejected
  defining the variants in plain CSS instead of Tailwind's classes;
  here the two variants keep their names, which is what that decision
  is about, and only their content changes form, as other rules of the
  stylesheet already write some properties in plain CSS.

## Results

Every acceptance test passed on 2026-11-15:

1. `style/input.css` writes `leading-normal` and `rounded-lg` where it
   wrote `leading-[1.5]` and `rounded-[8px]`; the editor shows no
   warning on them (checked by the user).
2. The built stylesheet, compared before and after, gives the same
   values: the line height is now `var(--leading-normal)`, which
   Tailwind defines as 1.5, the radius `var(--radius-lg)`, 0.5 rem,
   that is 8 pixels; three rules list their declarations in another
   order, none overriding another.
3. Every check of the pipeline passes: formatting, lints, 325 unit
   tests, the build, the type check and the browser tests (262 passed,
   1 skipped, with Playwright 1.65.0).
4. `footer-text-large` and `breadcrumb-separator` give the same CSS as
   before, and the editor flags neither (checked by the user).
