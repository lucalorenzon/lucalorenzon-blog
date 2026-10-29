# Name styles in the stylesheet

- **Opened:** 2026-10-29, raised by the user while reviewing the first
  attempt at the card
  [adapt the header to screen width](../wip/adapt-header-to-screen-width.md).
- **Status:** being worked on since 2026-10-29, before the code of the
  header card, so that code is written once with named styles.

## Why

Tailwind's standard way is "utility-first": each class in the HTML is one
small rule, and the HTML combines them. The generated stylesheet is
small and shared by every page, but what a sequence of classes is for is
written nowhere, and the screen-width thresholds stay implicit in
Tailwind's defaults (`lg:` means 1024px without saying so).

The user wants the code to say what each style is for: thresholds
declared with a name in `style/input.css`, combinations of styles
gathered into classes named after their purpose, and only those names in
the HTML. Tailwind v4 supports this (`@theme` for thresholds,
`@layer components` with `@apply` for named classes), still compiled
into a single stylesheet. Tailwind's own documentation advises using
`@apply` sparingly, so this goes against its recommended use and has to
be decided knowingly.

The rule applies to the whole site, not to one card, so it is recorded as
an architectural decision in `docs/adr/` with the `adr` skill.

## Questions

- Naming: what the thresholds and the classes are called.
- Whether everything moves to named classes, or only combinations that
  repeat or carry a meaning, with single utilities left in the HTML.
