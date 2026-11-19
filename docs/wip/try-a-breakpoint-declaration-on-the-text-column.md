# Try a breakpoint declaration on the text column

- **Created:** 2026-11-19
- **Status:** In work

## Why

The user opened this card on 2026-11-19, while taking up the card
[show the text column in the style guide](show-the-text-column-in-the-style-guide.md).
Both that page and the page of the breakpoints
([show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md))
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
  reader sees, not the class. Confirmed by the user on 2026-11-19.
- **The first component to try it on** is `OneColumnText`
  (`src/one_column_text.rs`), the only component of the site so far
  that changes at a breakpoint and is not the page frame.
- **The breakpoints are read from `config/breakpoints.txt`**
  (`src/style_guide/breakpoints.rs`), which also gives the Rust
  checks of the classes the prefixes they accept.
- **Few things change at a breakpoint today** (checked by the agent on
  2026-11-19): the text column, the page frame (header, logo, title,
  footer) and one sample on the style guide's colours page. The
  browser tests already check the column's behaviour on both sides of
  each breakpoint, so a list of changes out of step would make a page
  of the style guide wrong, not the site.

### How others face the problem

Searched on the web by the agent on 2026-11-19, at the user's request,
since the user asked whether others had faced the problem, or whether
it was not a problem at all. Three ways came up; none of the sources
found has a build that reads from the code the list of what changes
at each breakpoint.

1. **The documentation beside the code.** KSS writes the explanation
   in comments of the stylesheet, beside the classes, with each
   variant listed, and generates the style guide from them. Storybook
   keeps a file of documentation beside each component. Being near
   makes forgetting harder, but the declaration is still written by
   hand.
2. **Observing instead of declaring.** The most common answer to
   keeping the documentation in step: Chromatic photographs each
   component at several widths and reports every difference. A guide
   on responsive documentation advises showing the component side by
   side at each breakpoint, pointing out only what changes. An image
   made by the build cannot contradict the code.
3. **The breakpoint as a parameter of the component.** The Australian
   Department of Agriculture's design system gives its primitive
   components (the example is `Box`) responsive properties: the
   caller passes one value per breakpoint, as in
   `padding={{ xs: 1, sm: 2, md: 4 }}`, and the component turns it
   into media queries. SEEK's Braid does the same with `Box`, `Stack`
   and `Columns`. The declaration is not beside the view, it is the
   view. The sources show the pattern only on layout primitives,
   generic pieces used in many contexts, and give no reason for it.

Two more findings bear on the question:

- **Fewer breakpoints in components.** Several authors, after the
  book *Every Layout*, argue for intrinsic design: a component adapts
  to the room it has (fluid sizes, `min()`, `clamp()`), with a
  breakpoint only for what cannot change smoothly; and for container
  queries, in which a component reused in several contexts changes
  with the width of its container, not of the screen, while the
  screen's breakpoints are kept for the page's skeleton. These are
  opinion pieces that agree on a mix; no source settles the question.
- **The column's own case.** Its maximum width already follows its
  font with no class of its own; only its text size changes at a
  breakpoint.

Sources:
[Responsive Documentation](https://designsystemproblems.pages.dev/documentation-challenges/responsive-documentation),
[Agriculture Design System, breakpoints](https://design-system.agriculture.gov.au/foundations/tokens/breakpoints),
[Braid, layout](https://seek-oss.github.io/braid-design-system/foundations/layout),
[Chromatic, viewports](https://docs.chromaticqa.com/viewports),
[KSS](https://github.com/kneath/kss),
[Build a style guide straight from Sass](https://css-tricks.com/build-style-guide-straight-sass),
[Building a UI without breakpoints](https://frontendmasters.com/blog/building-a-ui-without-breakpoints/),
[Intrinsic layouts or container queries](https://frontendbytes.9elements.com/intrinsic-vs-cq).

## Questions to answer

The ones known when the card was written; there may be more. Raised
by the agent on 2026-11-19, from the user's idea, except where said.

- **Whether the direction taken goes against common practice**, and
  if so, whether there is a good reason to keep it. Raised by the
  user on 2026-11-19, after the search above: if others have solved
  the problem the other way, the user wants either to change position
  or to have a good reason to defend it.
- **Who decides what changes at a breakpoint**: the component itself,
  or whoever uses it, as with the responsive properties above. Raised
  by the agent on 2026-11-19.

  **The user's view on 2026-11-19, still under discussion.** The
  component: the caller deciding means the component's behaviour
  lives outside it, and the page's logic gathers in the caller, the
  opposite of modularity. The user wants to understand the reason
  behind the responsive properties before judging them.
- **Where the explanation lives**: in the trait's description, in the
  Markdown, or both, and then which says what.

  **The user's view on 2026-11-19, still under discussion.** The
  documentation stays with the code, as a principle (otherwise it
  would be in a repository of its own). The user had thought of a
  separate place, dedicated and linked to the component, to avoid one
  large file to open and scroll for every change; ready to revise
  this.
- **What keeps the declaration in step with the component's view**.
  The trait is written by hand too: if a class changes in the view
  and not in the trait, the two part, and the problem is only moved.
  Perhaps the build renders the component, collects the classes that
  carry a breakpoint's prefix and compares them with the declared
  ones; whether that is possible, and how a change with no class is
  checked, is to be found out. A third way came from the search: a
  single source, the declaration, from which the view takes its
  classes.

  **The user's view on 2026-11-19, still under discussion.** Text
  generated from the code would be best, but looked complicated here,
  so the user had fallen back on documentation written and kept in
  step by hand. This is at odds with the decision of 2026-11-18 that
  the build finds the changes.
- **Whether the column needs a breakpoint at all**, or could change
  its text size smoothly with the width, as intrinsic design suggests.
  Raised by the agent on 2026-11-19, from the search.
- **What a change is**: a name with its breakpoint and its classes,
  as the agent read it, or something else.

  **Answered by the user on 2026-11-19.** A named change, with its
  breakpoint and the classes that make it, sometimes none, as the
  agent read it. What else a change carries is for the spike to find.
- **How long the spike lasts**: its time limit, agreed when it is
  taken into work.

  **Answered by the user on 2026-11-19.** One working session, at
  most 200,000 tokens of the agent's context. When the limit is
  reached, the finding says what is known and what is still open.

  **Changed by the user on 2026-11-19.** Two working sessions, each
  within the same limit of tokens: the questions above are worth
  discussing, and one session would not be enough.

## References

- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  the first page that would use the mechanism; it waits for this
  finding.
- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the page that groups the changes by breakpoint.
- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md):
  the component the spike tries the mechanism on.
- [Show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md):
  the page whose Markdown is cut by headings, the model for the
  Markdown's side.
