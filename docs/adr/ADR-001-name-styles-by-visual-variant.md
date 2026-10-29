# ADR-001: Name styles by visual variant in the stylesheet

- **Date:** 2026-10-29
- **Status:** Accepted, as a trial: to be reviewed once the first pages
  are built with it
- **Card:** [name styles in the stylesheet](../done/2026-10-29_name-styles-in-the-stylesheet.md)

## Context

The site's look is written with Tailwind CSS. Tailwind's usual way is to
put many small style classes on each element of the page, each class
being one tiny rule ("centred text", "hidden", "bold"). It produces a
small stylesheet shared by every page, but reading the page source does
not tell what a group of classes is for, and the screen widths at which
the page changes are hidden in Tailwind's defaults (`lg:` means 1024
pixels without saying so).

The first attempt at a page that adapts to screen width was written that
way, and the user found it hard to read: it looked like a set of
workarounds rather than a clear description of how the page should
look. The user wants the code to say what each style is for.

## Decision

1. **The page carries only our own named styles.** Each name describes a
   visual variant, that is how the element looks, not what the element
   is: `title-below-line`, `title-beside-logo`, `header-one-row`,
   `logo-art-hidden`. Names are English, words joined by hyphens.
   Tailwind's built-in classes are not used directly in the page.
2. **A variant is switched on at a width with a prefix**, Tailwind's
   standard mechanism: `<h1 class="title-below-line xl:title-beside-logo">`
   reads "below the line; beside the logo from `xl` up". The page source
   shows which variant applies at which width.
3. **Variants are defined in `style/input.css`**, each written with
   Tailwind's classes. The name gives the readability; the details stay
   in Tailwind's vocabulary.
4. **Screen-width thresholds keep Tailwind's standard names**, `md`
   (768 pixels) and `xl` (1280 pixels), declared explicitly in
   `style/input.css`. Thresholds the site does not use are removed, so the
   file lists exactly the ones that exist.

Why: the name of a variant tells the reader what the element looks like
at each width, which is what the user missed in the page source, while
the page keeps Tailwind's mechanisms (width prefixes, one shared
stylesheet holding only the styles actually used) instead of replacing
them. "For now let's try": the user chose to try the strict form (only
our names in the page) and to review it after using it.

## Consequences

**Positive:**
- The page source says how each element looks and from which width.
- The thresholds are visible and their values written in one place.
- The stylesheet still holds only the variants actually used: Tailwind
  drops unused ones, as it does for its own classes.

**Negative / Risks:**
- Variants of a container and of its content depend on each other:
  `title-beside-logo` only works inside `header-one-row`. They must be
  switched at the same threshold, and a change to one may need a change
  to the other.
- Even trivial rules need a named variant (for example a variant just to
  hide the ASCII-art logo on small screens). If this becomes a burden,
  it is the signal to review this decision.
- Tailwind's documentation advises defining reusable classes with
  `@apply` sparingly. This decision goes against its recommended use,
  knowingly.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Name styles after the component (`page-title`), with its behaviour at every width inside the stylesheet | The name says what the element is but hides how it looks and when it changes, which is what the user wants to read |
| Tailwind's built-in classes straight in the page (Tailwind's usual way) | What a group of classes is for is written nowhere; thresholds stay implicit. This was the first attempt the user rejected |
| Define variants in plain CSS instead of Tailwind classes | Readability comes from the variant's name, not from its details; with plain CSS, Tailwind would bring little beyond thresholds and minification |
| Our own threshold names, such as `medium` and `wide` | More names to remember; some names make Tailwind generate stray classes (in a test, `tablet` produced an unused `.table` class) |
| Allow Tailwind's built-in classes in the page next to our names, for trivial rules | Mixes two kinds of names and needs a rule on when a combination deserves a name. Kept as the first option to consider if this trial is reviewed |

## Technical Notes

Tailwind v4 syntax, checked with the project's pinned Tailwind 4.3.3,
which generates the width-prefixed rule inside the matching media
query:

```css
@theme {
  --breakpoint-*: initial;
  --breakpoint-md: 48rem;   /* 768px */
  --breakpoint-xl: 80rem;   /* 1280px */
}

@utility title-below-line {
  @apply mt-6 text-2xl text-center;
}
```

`@utility` classes take width prefixes (`xl:title-beside-logo`) and are
left out of the stylesheet when unused, like Tailwind's own classes.

## References

- Card: [name styles in the stylesheet](../done/2026-10-29_name-styles-in-the-stylesheet.md)
- First use: [adapt the header to screen width](../done/2026-10-29_adapt-header-to-screen-width.md)
- Tailwind CSS v4: `@theme`, `@utility`, `@apply` ([documentation](https://tailwindcss.com/docs))
