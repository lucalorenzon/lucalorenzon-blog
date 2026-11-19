# Try a breakpoint declaration on the text column

- **Created:** 2026-11-19
- **Status:** Open

## Why

The user opened this card on 2026-11-19, while taking up the card
[show the text column in the style guide](../wip/show-the-text-column-in-the-style-guide.md).
Both that page and the page of the breakpoints
([show the breakpoints in the style guide](../wip/show-the-breakpoints-in-the-style-guide.md))
need each component to tell the build what changes in it at each
breakpoint, so that what the pages say cannot get out of step with
the code. How a component does it is not known, and the user said
the idea below was a brainstorm whose end they could not see. This
is a spike: a time limit and a written finding, no code that stays.
Without it, the mechanism would be designed inside the page of one
component, for every component to come.

## What is known

- **The user's idea** (2026-11-19): every component that changes at a
  breakpoint implements a trait that the build calls. The trait gives
  the build what it needs to compose the component's own page in the
  style guide and the breakpoints page: a unique code of the
  component, the Tailwind classes that change its look at a
  breakpoint, perhaps with a description, and the breakpoint each
  change belongs to, so the build can map breakpoints to components.
  On the Markdown's side, sections headed
  `## <component code> | <what it documents>`, in the way `fonts.md`
  is cut by the fonts' headings.
- **What changes is not only a class** (said by the user on
  2026-11-19): at a breakpoint a behaviour may change too. The column
  shows it: its text grows from 18 to 20 pixels through one class,
  `md:long-text-large`, while its maximum width grows from 600 to 720
  pixels with no class of its own, because it is 60 characters wide
  and the characters grew. The agent read from this that the unit is
  the change, named, with the classes that make it, sometimes none;
  so the heading would be `## <component code> | <change name>`. This
  agrees with the decision of 2026-11-18 to explain the change the
  reader sees, not the class. Not confirmed by the user yet.
- **The first component to try it on** is `OneColumnText`
  (`src/one_column_text.rs`), the only component of the site so far
  that changes at a breakpoint and is not the page frame.
- **The breakpoints are read from `config/breakpoints.txt`**
  (`src/style_guide/breakpoints.rs`), which also gives the Rust
  checks of the classes the prefixes they accept.

## Questions to answer

The ones known when the card was written; there may be more. Raised
by the agent on 2026-11-19, from the user's idea.

- **Where the explanation lives**: in the trait's description, in the
  Markdown, or both, and then which says what.
- **What keeps the declaration in step with the component's view**.
  The trait is written by hand too: if a class changes in the view
  and not in the trait, the two part, and the problem is only moved.
  Perhaps the build renders the component, collects the classes that
  carry a breakpoint's prefix and compares them with the declared
  ones; whether that is possible, and how a change with no class is
  checked, is to be found out.
- **What a change is**: a name with its breakpoint and its classes,
  as the agent read it, or something else.
- **How long the spike lasts**: its time limit, agreed when it is
  taken into work.

## References

- [Show the text column in the style guide](../wip/show-the-text-column-in-the-style-guide.md):
  the first page that would use the mechanism; it waits for this
  finding.
- [Show the breakpoints in the style guide](../wip/show-the-breakpoints-in-the-style-guide.md):
  the page that groups the changes by breakpoint.
- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md):
  the component the spike tries the mechanism on.
- [Show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md):
  the page whose Markdown is cut by headings, the model for the
  Markdown's side.
