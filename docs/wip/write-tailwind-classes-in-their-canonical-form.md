# Write Tailwind classes in their canonical form

- **Opened:** 2026-11-15, by the user, after Tailwind's CSS server in
  the editor flagged classes of `style/input.css` that have a shorter,
  standard name; to be done in the same session, so it is not
  forgotten.
- **Status:** in progress since 2026-11-15.

## Why

Tailwind's CSS server, which checks `style/input.css` in the editor
(`.zed/settings.json`), suggests a canonical name for a class written
with an arbitrary value that equals a value of Tailwind's own scale.
The suggestions stay on screen as warnings, and a warning that is
always there hides the one that matters. The canonical name also says
that the value is one of the scale's, not a value chosen for one place.

## Known from the start

The server flagged five classes on 2026-11-15:

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

So the change is to the stylesheet's source only: what the browser
draws stays the same.

## Questions

- Whether the two false alarms are silenced in the editor's settings,
  or left on screen.

## Acceptance tests

1. `style/input.css` writes `leading-normal` and `rounded-lg` where it
   wrote `leading-[1.5]` and `rounded-[8px]`, and the server flags none
   of the four places.
2. The built stylesheet gives the same line height (1.5) and the same
   corner radius (8 pixels) to the rules that used them.
3. Every check of the pipeline passes, the browser tests included.
4. The two false alarms are answered as the question above decides.
