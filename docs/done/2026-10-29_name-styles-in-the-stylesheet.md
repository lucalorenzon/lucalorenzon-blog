# Name styles in the stylesheet

- **Opened:** 2026-10-29, raised by the user while reviewing the first
  attempt at the card
  [adapt the header to screen width](2026-10-29_adapt-header-to-screen-width.md).
- **Status:** done on 2026-10-29. Worked on before the code of the header
  card, so that code was written once, with named styles.

## Why it was opened

Tailwind's standard way is "utility-first": each class in the HTML is one
small rule, and the HTML combines them. The generated stylesheet is
small and shared by every page, but what a sequence of classes is for is
written nowhere, and the screen-width thresholds stay implicit in
Tailwind's defaults (`lg:` means 1024px without saying so). The user
wanted the code to say what each style is for.

## Answers

The rule applies to the whole site, so it is recorded as
[ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md); in short:

- **What a name describes:** how the element looks, its visual variant
  (`title-below-line`, `title-beside-logo`), not what the element is
  (`page-title`). The user's proposal: a component name hides how the
  element looks and when it changes.
- **How a variant changes with width:** with Tailwind's standard prefix,
  in the HTML: `title-below-line xl:title-beside-logo`. A single name
  holding every width would have become a component name again.
- **What the HTML may contain:** only our named variants, no Tailwind
  class directly, "for now": a trial, to review if naming trivial
  variants (such as `not-shown`) becomes a burden. Rejected for now:
  allowing Tailwind's classes for trivial rules, which mixes two kinds
  of names.
- **What a variant is written with:** Tailwind's classes, through
  `@apply`. The user: readability comes from the variant's name, not
  from its details. Rejected: plain CSS, with which Tailwind would bring
  little.
- **Threshold names:** Tailwind's own, `md` (768px) and `xl` (1280px),
  declared explicitly in `style/input.css`, the unused ones removed.
  Rejected: names of our own, such as `medium` and `wide`, which are more
  to remember; in a test, the name `tablet` made Tailwind generate an
  unused `.table` class.

## Results

- The header's code uses thirteen named variants; a unit test fails if
  the page carries a class that is not defined as a variant in
  `style/input.css`, so the rule is checked on every build.
- Reviewing the generated stylesheet showed about fifteen classes no page
  used (`.container`, `.table`, `.mt-6`…), present since before this
  card: Tailwind scanned the whole repository on its own and picked up
  words from the documentation. The user asked for Tailwind to look only
  at the files that matter: it now scans only the files listed in
  `style/input.css` (`source(none)`), and the stylesheet holds only the
  variants used, 6.8 KB instead of 9 KB.
