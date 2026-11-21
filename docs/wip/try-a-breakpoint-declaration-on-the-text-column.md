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
[Intrinsic layouts or container queries](https://frontendbytes.9elements.com/intrinsic-vs-cq),
[Tailwind, responsive design](https://tailwindcss.com/docs/breakpoints),
[CSS Containment Module Level 3](https://w3.org/TR/css-contain-3/),
[OddBird, containment](https://css.oddbird.net/rwd/query/contain/).

### What the experiment with a container query found

Run on 2026-11-20 by a sub-agent (Claude Sonnet 5.5), on a throwaway
branch, `worktree-agent-afb8353149f05acd9` (commit `d018d05`, based on
`6ed629e`), deleted when the spike closes; the agent checked the
generated rule and the samples' screenshot. The change tried: the
frame's `div#top` made a container by a new utility,
`page-container` (`container-type: inline-size`); a container size
`--container-column: 48rem` (768 pixels) in a `@theme` block; the
column's class `@column:long-text-large` instead of
`md:long-text-large`. Before the change, 423 unit tests and 275
browser tests passed.

1. **Tailwind 4.3.5 supports it** with no plugin: the stylesheet gets
   `@container (min-width:48rem){.\@column\:long-text-large{...}}`.
   If the container size is not defined, or its name is mistyped, the
   rule is left out with no error; the Rust checks see only the class
   name, so neither would notice.
2. **The Rust checks of the classes refuse the new classes**: six
   tests failed, on `@container`, a class of Tailwind and not of the
   stylesheet, and on the `@column:` prefix, unknown to
   `without_prefix`. The sub-agent used a utility of the site for the
   container and wrote the prefix into the code by hand, a shortcut;
   the real answer would read the container sizes from `config/`, as
   the breakpoints are.
3. **The column's browser test passes unchanged**, but the behaviour
   is not the same. A container does not include the scrollbar, the
   screen's breakpoint does. With a classic scrollbar, 15 pixels wide,
   on a page that scrolls, the text grows to 20 pixels at a window of
   783 pixels instead of 768; in between, the header, still following
   the screen, and the text, following its container, disagree. The
   test does not see it, because the tests' headless Chromium hides
   the scrollbars; seeing it needs the option
   `ignoreDefaultArgs: ['--hide-scrollbars']`.
4. **No side effect found**: every unit test and every browser test
   passes, and screenshots of the home page and of the text page, at
   375 and 1280 pixels, are identical pixel for pixel before and
   after. A container that already fills the width changes nothing in
   the layout.
5. **The column shown at several widths on one page works**: in
   containers of 400, 700 and 900 pixels, with a window of 1280, the
   text is 18, 18 and 20 pixels (the columns 400, 600 and 720 wide).
   But each sample is a `<main>`, and a page should have one: the
   on-demand check of axe's best practices reports
   `landmark-no-duplicate-main` and `landmark-unique`, while the
   accessibility test of the build, which runs WCAG's rules only,
   does not. A sample would need the component to write another
   element, such as a `<div>`. A sample of fixed width also scrolls
   the page sideways on a 320-pixel screen.
6. **What changes in the built site**: 14 pages, each in its two
   classes only, and 79 bytes more in the stylesheet (28,590 to
   28,669).

Found without being asked:

- **Tailwind's default container sizes are active** in this project,
  since the build clears only the screen's breakpoints: `@3xl` is
  already 768 pixels. Naming the size `md` would work, and would not
  clash with the screen's `md:`, but would change `max-w-md` silently;
  a name of the site's own, such as `column`, avoids it.
- **A container without a name is the one any descendant queries.**
  The code blocks and the tables already use named containers
  (`code`, `table`); a column placed inside one of them would query
  it. A named container for the page would avoid it.

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

  **Discussed on 2026-11-19, still under discussion.** The agent found
  that the responsive properties are given only to layout primitives,
  generic pieces that cannot know what a page needs; a component of
  the site, which holds the site's own choices, decides for itself. In
  this site the primitives are, today, the classes of
  `style/input.css`: `md:long-text-large` is the component passing
  its classes one value per breakpoint. The nearest to a primitive
  component is the colours page's sample,
  `sample-stacked md:sample-side-by-side`, a generic "stacked, then
  side by side".

  The agent described what `OneColumnText` holds: the measure (60
  characters), the text size (18, from `md` 20), the page's `<main>`,
  and the look of every element of the text, over a hundred lines of
  `long-text-column`; the generic part is a centred column of a
  maximum width. The user observed that it is then not generic because
  it is cut wrong: it took over things that are not about "one
  column". The refactor kept the HTML byte for byte, so the component
  took the shape of the class that was already there. The agent named
  two tensions: the name the user chose says "text" too; and the
  column's width is counted in characters of its own font, so width
  and text size are one thing for the stylesheet.

  The user's view then (still not clear-cut): the site sets the rules,
  so that it is uniform: margins, spacing, fonts, and the sizes of
  text to choose from; the component decides how it takes its space,
  that is which of the site's sizes it uses for the space it has. The
  agent noted that this is what design systems call design tokens,
  and that the site does it already for colours, fonts and
  breakpoints, not for text sizes, written inside the column's
  classes.
- **Whether a component changes with the screen or with the space it
  has in the page.** Raised by the agent on 2026-11-19, from the
  user's words "the space it has".

  **The user's view on 2026-11-19, still under discussion.** The space
  the component really has in the page, said on purpose, though the
  user is not fully convinced and sees that it goes against what was
  said before; it comes from reading the articles found in the search.
  The agent confirmed the reading: these are container queries, and
  the sources agree on using them for components reused in several
  contexts, keeping the screen's breakpoints for the page's skeleton.
  Their support in browsers is stable since 2023, and Tailwind 4 has
  them with no plugin (`@container` on the parent, `@md:` on the
  children, sizes from `--container-*` in `@theme`). What the agent
  saw coming: an element queries an ancestor declared a container,
  never itself, so the page frame would declare the containers; a
  container's width cannot depend on its content, so one that shrinks
  to fit it collapses; two kinds of thresholds to explain, `md:` and
  `@md:` with different values; the browser tests vary a container
  rather than the window. And one gain: the style guide could show
  the column in containers of several widths side by side, which the
  column's page asks for.

  **The user decided on 2026-11-19** that the spike tries the column
  with a container query, instead of the declaring trait. What the
  try found is under "What the experiment with a container query
  found" above. The agent's reading on 2026-11-20, not discussed yet:
  the threshold becomes the component's own, and a page of the
  breakpoints valid for the whole site shrinks to the page's skeleton,
  the header and the footer; keeping a list of changes in step could
  give way to the second way of the search, observing instead of
  declaring: the column's page shows the component at each of its
  thresholds, and a test checks that what it shows is true. Adopting
  container queries changes behaviour (the scrollbar), so it would be
  a card of its own, not a refactor.
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
