# Show the page frame in the style guide

- **Created:** 2026-11-21
- **Status:** In work

## Why

Opened by the agent on 2026-11-21, at the close of the spike
[try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md),
with the user's agreement. The spike found that a component does not
declare what changes in it at a breakpoint: its page in the style
guide shows it at each of its thresholds, and a test checks that
what it shows is true. So the page of the breakpoints, which was to
list the changes of the whole site
([show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md)),
was superseded. The page frame (header, logo, title, footer) is the
only part of the site left on the screen's breakpoints, and had no
page to show its changes.

## What is known

- **What the frame changes at each breakpoint** (`src/page_frame.rs`,
  checked by the agent on 2026-11-22): one change only, at 1280
  pixels, where the header goes from two rows (the logo and the menu's
  room above the line, the title below it) to one row, with the title
  between the logo and the menu's room. Until the logo was reviewed
  ([card](review-the-logo.md), closed on 2026-11-22) there was a
  second change, at 768 pixels, where the written logo gave way to the
  logo drawn in characters; the new logo, a round icon with the name
  beside it, is the same at every width. Its text grows within ranges of its own over
  the page's widths from 320 to 1366 pixels, written in the
  stylesheet (`style/input.css`): the title from 24 to 32 pixels, the
  footer's text from 16 to 18; the section headings are 1.2 times the
  column's text
  ([card](let-the-frame-text-grow-within-the-injected-ranges.md),
  done on 2026-11-22).
- **The frame writes the text column** since 2026-11-21: the
  `<main>`, with the look of the text, went back to it from the
  component `OneColumnText`, removed
  ([decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md)).
  The column's own page
  ([show the text column in the style guide](show-the-text-column-in-the-style-guide.md))
  joined this one, decided by the user on 2026-11-21, at least until
  the user changes their mind. What was known there:
  - **What changes in the column** (checked by the browser test
    `text-column.spec.ts`): its text no longer jumps at 768 pixels; it
    grows on a straight line from 18 pixels at a 320-pixel page to 20
    pixels from 1366 up, written in the stylesheet
    ([card](let-the-column-text-grow-within-an-injected-range.md),
    done on 2026-11-22). Its maximum width, 60 characters of its own
    font, grows with its text. Below its maximum it fills the screen;
    it is always centred. Until 2026-11-22 the text was 18 pixels
    below 768 and 20 from 768, and the column 600 and 720 pixels wide
    with Literata.
  - **The browser test of the column moves here**: written on the
    style guide's text page, it moves to this page once it exists
    (agreed by the user on 2026-11-19, for the column's page).
  - **The style guide's text page** already shows the elements of the
    text inside the column
    ([card](show-the-text-elements-in-the-style-guide.md)), and the
    tables page names the column's width in passing.
- **Showing the frame at several widths on one page** would
  duplicate its header, footer and main heading, as the spike's
  experiment duplicated `<main>`. An `iframe` has a window of its own,
  so the screen's breakpoints switch at its width, and is a document
  of its own, so it duplicates nothing in the page that holds it.
  Proposed by the agent in the spike, not tried.
- **Decisions of the breakpoints page that still hold** (see that
  card): the style guide is published with its gaps declared; the
  components not yet cut in the code are declared as a limit of the
  site as it is; the pages the user wishes for later are the title,
  the logo, the header, the footer, the text column, the list of
  articles and the article's box in that list.
- **The breakpoints sort the devices into groups**, three today, and
  the breakpoints page was to show the devices of each group, taken
  from the list the browser tests run on.

## Questions to answer

The ones known when the card was written; there may be more. Three
more came from the breakpoints page's card, still open when it was
superseded; they left this card on 2026-11-22 (see "What it leaves").

- **Whether the `iframe` passes the accessibility checks**, the
  build's and axe's best practices, and how wide a sample may be on a
  320-pixel screen without scrolling the page sideways.

  **Answered by the user on 2026-11-22, with the agent: no `iframe`.**
  Once the tests check the frame of the page itself (the next
  question), the samples would serve the reader only. The agent set
  out the two ways: samples side by side, which show the two-row and
  one-row header at a glance but bring in the `iframe` and its checks;
  or an explanation of what changes and at which width, which the
  reader sees by resizing the window, though a reader on a phone never
  sees the one-row header. The user chose the explanation: the frame
  around the text is the real one, and an `iframe` would be awkward to
  look at on a phone anyway.
- **What the page puts to the test.** The frame's browser tests run
  today on the home page, which does for the frame what the two
  articles did for the text's components before the style guide took
  them over
  ([ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md)).
  The frame's page is itself inside the frame, so its own header and
  footer are what is tested, with no sample.

  **Answered by the user on 2026-11-22, with the agent.** The tests
  that check the frame as a component leave the home page for the
  frame's page: the header's layout at every width, the line under
  the logo's name, the footer's text size, the header with the
  reader's text spacing, and the frame's accessibility and contrast.
  The text column's tests join them from the style guide's text page,
  as agreed on 2026-11-19. Stay where they are: the net that runs on
  every page of the sitemap (the logo on every page, the shape of
  each page), as ADR-004 says, and the tests of the home page's own
  content (its list of articles and their links). The "page not
  found" stays out of the move: it is a page of its own, with links
  written for addresses of any depth, though tested today with the
  home page for the frame's accessibility and contrast. The agent
  checked that the move needs nothing of the build: the header's
  test puts its two invented longest titles in place of the heading
  in the browser, so it works on any page.
- **Where the explanation of a component lives**: beside its code, or
  in the Markdown of `content/style-guide/` as for the other pages.
  Came from the column's page. The user's view on 2026-11-19: the
  documentation stays with the code, as a principle; the user had
  thought of a separate place, dedicated and linked to the component,
  to avoid one large file to open and scroll for every change; ready
  to revise this.

  **Answered by the user on 2026-11-22, with the agent: beside the
  code.** The agent set out the tension: every other page of the
  style guide has its text in `content/style-guide/`, and nothing
  there ties it to the code it describes. Asked for an example, the
  agent proposed a Markdown file beside the component,
  `src/page_frame.md`, taken into `src/page_frame.rs` twice: as the
  module's documentation (`#![doc = include_str!("page_frame.md")]`)
  and as the text the style guide's page reads. One text is then the
  component's documentation and its page, kept small and apart from
  the code, and in the same folder and the same change as the code it
  describes. The agent named four consequences, none blocking: the
  text leaves the licences of `content/` for the code's; the content
  dates step reads only `content/`, so the page would have no date in
  the sitemap; two rules, the content's elements in `content/` and
  the page's components beside their code; the check that the
  Markdown holds no HTML has to reach the new file. The user chose
  the file beside the code, and asked for a card that brings the
  other pages to the same model after publishing
  ([card](move-the-style-guide-texts-beside-their-code.md)). Recorded,
  since it spans several cards, in
  [ADR-005](../../adr/ADR-005-explain-a-component-beside-its-code.md).
- **Under which licence the explanation beside the code is**: the
  code's, MIT, or the text's, as an exception declared in `src/`.
  Added on 2026-11-22, from the answer above. The style guide's pages
  say "Reuse with attribution" in their footer, which leads to the
  site's text licence (CC BY 4.0, under "unless a page says
  otherwise" in `content/licence.md`), while everything in `src/` is
  under MIT: the visitor and the reader of the repository would be
  promised different terms for the same text.

  **Answered by the user on 2026-11-22, with the agent: the text's
  licence.** The user first asked whether the code could move to
  Creative Commons too, to make things simpler. The agent advised
  against it: Creative Commons' own frequently asked questions say
  that software and hardware are the only works for which it does not
  recommend its licences (read verbatim by the agent on 2026-11-22);
  the entry on software, read only in a summary, adds that the
  licences say nothing on source code or patents and are not
  compatible with the main software licences. The repository would
  keep several licences anyway (the robots list under MIT, the
  articles' code samples under MIT-0), and a licence once given
  cannot be taken back from those who took the code. The agent
  proposed one rule instead, which the user accepted: every Markdown
  file that explains a component is text of the site, under the same
  licence as the style guide's pages, wherever it lives; one line in
  `content/README.md`, and perhaps one in the root `README.md`, says
  that the `.md` files in `src/` follow the text's licences. The code
  stays under MIT, and the page keeps its footer's promise. Creative
  Commons calls its licences fit for software documentation (also
  from the summary only).
- **Where the page's date in the sitemap comes from**, since the
  content dates step reads only `content/`. Added on 2026-11-22, from
  the answer above. Each page of the sitemap takes the date of the last
  commit of its Markdown file (`src/sitemap.rs` names the file,
  `scripts/content-dates/content_dates.py` dates it); reaching the
  `.md` files in `src/` is a small change. The question is what the
  date tells, since the frame's page shows the frame itself: a change
  of `src/page_frame.rs` changes what the page shows, even when its
  explanation stays the same.

  **Answered by the user on 2026-11-22, with the agent: the date of
  the text**, as for the other pages. The agent set out the two
  readings: the text's date, consistent with the other pages and with
  the sitemap's purpose of telling search engines that the content
  changed, though a visible change of the frame leaves the date as it
  is; or the date of the component, text or code, closer to what the
  visitor sees, though any change of the code inside, also an
  invisible one, would make the page look new. So the content dates
  step also reads the `.md` files in `src/`, and the page's source in
  the sitemap is `src/page_frame.md`.
- **Who writes the page's text.** The other pages of the style guide
  have the author's text, except one section of the colours page, a
  placeholder declared as such. Added on 2026-11-22, once the card was
  cut to the page itself.

  **Answered by the user on 2026-11-22: the agent writes a draft**,
  declared as a placeholder as on the colours page, and the user
  rewrites it afterwards.
- **Where the boundary lies with the text page**, which shows what
  goes inside the column. Came from the column's page. The frame's
  page shows the column (its width, its text growing from 18 to 20
  pixels, its centring); the text page shows the elements inside it.
  Two cases sit across the line, found by the agent on 2026-11-22:
  the section headings' size, set at 1.2 times the column's text by
  the frame's ranges
  ([card](let-the-frame-text-grow-within-the-injected-ranges.md)) but
  checked on the text page with the room around the headings
  (`content-layout.spec.ts`); and the size the column passes on to
  the code block and the table (`text-column.spec.ts`), checked on
  their pages.

  **Answered by the user on 2026-11-22, with the agent.** The size the
  column passes on stays on the code and tables pages: only there do
  those components exist. The section headings' size belongs to the
  frame: the user saw it, with a confidence of 60%, as a style rule the
  frame injects to keep the site uniform. The agent agreed, with the
  rule that the place goes to whoever decides the value: the ratio of
  1.2 is written by the frame in the stylesheet, beside the other
  sizes that grow with the page, as the column's size passed on to
  the code block. Following it through, the agent proposed splitting
  the section heading in two, which the user accepted. On the frame's
  page, the rule injected: section headings are 1.2 times the
  column's text at every width, measured on the page's own headings,
  with no sample. On the text page, what belongs to the heading: its
  colour, the room around it, how it wraps, the underline of level
  four; the test that checks a section heading is larger than the
  text gives up the size there and keeps the rest. If the ratio
  changes, the test that breaks is the frame's, where the rule is
  written.
- **Whether the frame is one page, or the header, the logo and the
  title are pages of their own**, whose boundaries were still to be
  drawn. For the logo, the same question is asked by the card
  [decide whether the logo is a component of its own](decide-whether-the-logo-is-a-component.md),
  opened on 2026-11-22 once the logo was reviewed: the answer here
  answers it there too.

  **Answered by the user on 2026-11-22, with the agent.** One page, the
  frame shown whole at its widths; its parts are not cut into
  components of their own for now. The site is static HTML today, and
  the user expects logic soon: background images, a header that is
  transparent and turns opaque when the text passes under it,
  parallax for depth, the title and the logo's name moving when the
  title reaches the header, the articles' cards scrolling sideways,
  perhaps two columns on wide screens. Those behaviours are expected
  to give the components their shape; cutting them now would draw
  boundaries from the wrong information. The agent agreed, adding
  that a component is cut along the parts that change together, and
  those behaviours tie the logo, the title and the header together:
  cut now along the static layout, they would likely be put back
  together, as the text column was. The agent added one point: the
  page's tests check what the visitor sees, not the code's
  boundaries, so they stay true after a later cut and guard it.

## Dependencies

- **Waits for the logo**
  ([review the logo](review-the-logo.md)), decided by the user on
  2026-11-22: the logo's change at 768 pixels, one of the two changes
  this page would show, is expected to go, so the page is made once
  the logo is settled. Met on 2026-11-22: the logo's card and the
  card that hid the header's line under the logo's name
  ([card](hide-the-header-line-under-the-logo-name.md)) are closed,
  and the change at 768 pixels is gone.

## Acceptance tests

Agreed with the user on 2026-11-22, for the page itself.

| # | What is checked | How |
|---|---|---|
| 1 | The build writes `style-guide/frame.html`, titled "Page frame", with the text of `src/page_frame.md`; like the other pages of the style guide it has its tab title, a heading within the frame's limit, a description within 160 characters, its canonical link, its Open Graph tags, the footer "Reuse with attribution", the link to the index and the way back to the top | Unit tests, on the model of the code page's |
| 2 | `src/page_frame.md` is also the documentation of the module `src/page_frame.rs` | The compiler; `cargo doc` tried once |
| 3 | The index lists the frame's page after the fonts | Unit test of the index |
| 4 | The sitemap lists the page with the date of the last commit of `src/page_frame.md`; the content dates step also reads the `.md` files in `src/` | Unit tests, in Rust and in Python |
| 5 | The page's Markdown holds no HTML | Unit test |
| 6 | The draft is declared a placeholder, in italic as on the colours page, and explains the header's change at 1280 pixels, the growth of the title, the footer and the column's text, and the section headings at 1.2 times the text | Review by the user |
| 7 | `content/README.md`, and one line in the root `README.md`, say that the `.md` files in `src/` follow the text's licences | Review by the user |
| 8 | The net passes on the new page with no test written for it (accessibility, contrast, shape, the search engines' rules, Open Graph); every check of the pipeline passes: formatting, clippy, unit tests, type check, browser tests | The pipeline's checks, run locally |
| 9 | Before the commit, the user sees the real page in screenshots, on the small screen and on the laptop | Screenshots with the project's Playwright |

The agent noted on 2026-11-22 that it had said wrongly, earlier the
same day, that the check that the Markdown holds no HTML reads
`content/`: it reads only the articles' bodies
(`src/article_page.rs`). Test 5 covers this page; whether the other
pages need it is left to the card that moves their texts.

## What it leaves

Cut by the user on 2026-11-22, to keep the card on what publishing
the style guide needs:

- Whether the list of devices can come from Firefox's, and what the
  tests' 320 is called: they concern every browser test, not this
  page. Card
  [follow a known device list in the browser tests](follow-a-known-device-list-in-the-tests.md).
- How the index declares the pages to come: it concerns the index,
  and is needed before publishing. Card
  [declare the pages to come in the style guide's index](declare-the-pages-to-come-in-the-style-guide-index.md).

Cut by the user on 2026-11-22, once every question had its answer, so
that this card keeps one slice: the page itself (its explanation
beside the code, its place in the style guide, in the index and in
the sitemap with the text's date, and the line on the licence). The
tests that move onto it went to two cards:

- The frame's tests, from the home page. Card
  [move the frame's tests to its page](move-the-frame-tests-to-its-page.md).
- The text column's tests, from the text page, and the section
  heading's test split in two. Card
  [move the text column's tests to the frame's page](move-the-column-tests-to-the-frame-page.md).

## References

- [Try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md):
  the spike whose finding this page applies.
- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the page this one replaces, and the decisions it inherits.
- [Let the page frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  what the frame's text becomes.
- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  the other component page born of the spike, joined to this one.
- [Decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md):
  the card that put the column back into the frame.
- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  what the column's text becomes.
- [Show the text elements in the style guide](show-the-text-elements-in-the-style-guide.md):
  the text page, the closest to the column.
- [Build a design system](build-a-design-system.md): the card
  the style guide belongs to.
- [Review the logo](review-the-logo.md): a page of the logo would
  describe something that may change; closed on 2026-11-22.
- [Hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md):
  the last change to the header before this page.
- [Decide whether the logo is a component of its own](decide-whether-the-logo-is-a-component.md):
  the same question as this card's on the frame's pages, from the
  logo's side.
- [Follow a known device list in the browser tests](follow-a-known-device-list-in-the-tests.md)
  and
  [declare the pages to come in the style guide's index](declare-the-pages-to-come-in-the-style-guide-index.md):
  the cards cut from this one.
- [Move the style guide's texts beside their code](move-the-style-guide-texts-beside-their-code.md):
  the other pages brought to this page's model, after publishing.
- [ADR-005](../../adr/ADR-005-explain-a-component-beside-its-code.md):
  the record of the decision on where a component's explanation lives.
- [Move the frame's tests to its page](move-the-frame-tests-to-its-page.md)
  and
  [move the text column's tests to the frame's page](move-the-column-tests-to-the-frame-page.md):
  the two slices of tests cut from this card.
