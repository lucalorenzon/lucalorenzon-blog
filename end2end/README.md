# Browser tests

The site is tested in a real browser (Chromium, driven by
[Playwright](https://playwright.dev/)): the tests open the pages and
check what a reader sees, never how the page is built. This file lists
every test: what it protects the site from, why it exists, and where to
find it in the code.

## How to read the tables

The tests are grouped first by **what they protect from**, then by the
**class** that
[ADR-004](../docs/adr/ADR-004-test-components-once-and-pages-with-a-net.md)
gives them. The class decides how often a test runs:

| Class | What it checks | Runs on |
|---|---|---|
| **Integration (net)** | What exists only in a composed page, or depends on its text | Every page of the sitemap, at one width, in one theme. ADR-004 and the code call it "the net". |
| **Component** | One component's behaviour: code block, table, list, heading | Once, on the case that stresses it most |
| **Frame** | The site's own pages and the frame every page shares: header, logo, footer, home page, "page not found" | The pages concerned, at the widths that change them |
| **Guard** | The tests themselves: that a shortcut the suite takes is still valid | Once |

Each table is shaped like a FitNesse decision table. The columns
without a question mark say what the test runs on; the column with one
says what makes it fail.

| Column | Meaning |
|---|---|
| Find in files | A piece of the line that declares the test, as the code writes it. It appears only once in `end2end/tests/`: search for it to land on the test. |
| File | The file in `end2end/tests/`. |
| Pages | The pages the test opens. "Build only" or "live only" when it runs in one of the two moments (below). |
| Widths | The screen widths, in pixels. "Column widths" are 320, 393, 412, 767 and 768: one per size of the text column. "Bands" are 393, 768 and 1280: one per form of the header. "Every device" is the twelve widths from 320 to 1920. |
| Themes | Light, or light and dark. Only tests on colours run in both themes, since the dark theme changes nothing else (a guard checks it). |
| Fails when? | What the test rejects. |
| Why | What a reader would lose if it were not checked. |

The component tests run on the style guide's pages, whose cases are
written for them: [build a design system](../docs/wip/build-a-design-system.md).

## When the tests run

- **On every build,** on the site just built in `target/site`, locally
  and in the pipeline (`.github/workflows/build.yml`).
- **After each publication,** on the live site
  (`.github/workflows/deploy.yml`, with `BASE_URL` set). The tests that
  walk the built folder, or open the "page not found" as a file, skip
  there.

To run them locally, from the repository root:

```sh
scripts/build-site.sh
cd end2end && npx playwright test
```

To run one test, give its file and words of its title:
`npx playwright test tests/readability.spec.ts -g "visual presentation"`.

## Keeping this file in step

This file must list every test the code declares, once. It is a rule of
the project (`CLAUDE.md`, "The browser tests' README"), and a check in
the build enforces it: `readme.spec.ts`, the last row of this file. The
check sees only the first column; "Fails when?" and "Why" stay true only
if they are reviewed whenever a test's behaviour changes.

## Barriers for disabled readers

Faults that keep out a reader who uses a screen reader or a keyboard,
or who sees colours poorly.

### Integration (net)

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `accessibility of the ${pageName}` | `accessibility.spec.ts` | every page of the sitemap | 1280 | light | axe finds a violation of WCAG 2.0 to 2.2, levels A and AA plus the AAA rules it can check, or of the three heading rules (one level-1 heading, heading order, no empty heading) | The text of each page changes the result: an image without a name, a skipped heading level |
| `contrast of the ${pageName}` | `accessibility.spec.ts` | every page of the sitemap | 1280 | light and dark | a text falls below a contrast of 7:1 (4.5:1 for large text), the drawn logo excepted | WCAG 1.4.6, level AAA: the site chose the highest contrast ([ADR-003](../docs/adr/ADR-003-readability-first-amber-as-accent.md)) |
| `shape of the ${pageName}` | `accessibility.spec.ts` | every page of the sitemap | 1280 | light | the banner does not hold exactly the logo link and one level-1 heading; the footer does not hold the copyright and one link; the main text holds a level-1 heading | axe does not notice what a screen reader is given: a logo that loses its name, parts read in another order |
| `no identifier is used twice` | `accessibility.spec.ts` | every page of the sitemap | 1280 | light | two elements share an `id` | A label pointing to a repeated identifier reads the wrong element; axe no longer checks it since WCAG 2.2 |

### Component

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `accessibility of the code blocks` | `code-blocks.spec.ts` | style guide, code blocks | 320, 768 | light | axe, limited to the code blocks, finds a violation | The toggle exists only at some widths, so the net, at one width, does not see it; the page has blocks with and without it |
| `contrast of the code blocks` | `code-blocks.spec.ts` | style guide, code blocks | 320, 768 | light and dark | a text of the code blocks falls below the contrast required | As above, for colours |
| `Tab reaches it and Space switches it` | `code-blocks.spec.ts` | style guide, code blocks | 320 | light | Tab does not reach the wrap toggle, or Space does not switch it | A reader without a mouse must be able to wrap the code |
| `the focus outline of the wrap toggle is whole` | `code-blocks.spec.ts` | style guide, code blocks | 320, 1280 | light | the toggle has keyboard focus and its outline is missing, or the checkbox draws its own (the mask would cut it), or any side of the outline (label, offset and width included) is outside the block's scroller or the page | The scroller's edge cut the outline at the top and left in the trials; a keyboard reader must see where the focus is |
| `the wrap toggle is an icon: no text` | `code-blocks.spec.ts` | style guide, code blocks | 320 | light | a block that fits shows the toggle; or the toggle shows text, is smaller than 24 by 24 px, lacks the tooltip "Wrap long lines of code" on its label, or has a title on the checkbox; or the checkbox is not a checkbox named "Wrap long lines of code", has an accessible description, or does not follow its state | An icon has no word: its name, state and tooltip must come from the markup, and the name must not be read twice (WCAG 2.5.8, 4.1.2) |
| `the wrap toggle off is the icon in the code accent` | `code-blocks.spec.ts` | style guide, code blocks | 320 | light and dark | off, the checkbox is not drawn in the code accent or the label has a fill; on, the label is not filled in the code accent or the icon is not in the code surface's colour | The fill is the only sign of the state a sighted reader has, and it must hold in both schemes |
| `the wrap toggle in forced colours mode` | `code-blocks.spec.ts` | style guide, code blocks | 320 | light | with forced colours emulated, the checkbox or the label does not keep its own drawing (`forced-color-adjust: none`); off, the icon is not in `CanvasText` or the label has a fill; on, the icon is not in `HighlightText` or the label not in `Highlight` | The mode forces the backgrounds to the page's, so the masked icon and the fill vanished: the readers who asked for their own colours lost the toggle. Forced colours ignore the scheme, so one theme is enough |
| `accessibility of the tables` | `table-forms.spec.ts` | style guide, tables | 767, 768 | light | axe, limited to the tables, finds a violation in either form | The record viewer exists only on narrow screens: at 767 the page shows both forms, at 768 the table only |
| `contrast of the tables` | `table-forms.spec.ts` | style guide, tables | 767, 768 | light and dark | a text of the tables falls below the contrast required | As above, for colours |
| `the keyboard reaches the viewer of` | `table-forms.spec.ts` | style guide, tables | 767 | light | Tab does not reach the record viewer of the six-column table, or the down arrow does not show its second record, or Page Down its third | A reader without a mouse must be able to read every record |

### Frame

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `accessibility of the frame, ${label}` | `accessibility.spec.ts` | home page; "page not found" (build only) | bands | light | axe finds a violation, as in the net | The header changes form with the width (text logo, drawn logo, title below or beside it), and the net sees only one |
| `contrast of the frame, ${label}` | `accessibility.spec.ts` | home page; "page not found" (build only) | bands | light and dark | a text falls below the contrast required | As above, for colours |

## Text hard to read or to copy

Faults that make the text tiring or impossible to follow, or that
change it when it is copied. Most come from WCAG 1.4.8 (visual
presentation) and 1.4.12 (text spacing).

### Integration (net)

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `no line over ${MAX_CHARACTERS} characters` | `readability.spec.ts` | every page of the sitemap | 1280 | light | a line, as the browser breaks it, is longer than 80 characters | WCAG 1.4.8: long lines lose the reader's eye. It depends on the text, since the font is proportional |

### Component

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `visual presentation (WCAG 1.4.8)` | `readability.spec.ts` | style guide, text | column widths | light | line height under 1.5 times the text size; paragraphs closer than 1.5 lines; justified text; or a case of the page takes one line at that width, so there is nothing to measure | WCAG 1.4.8: dense or justified text is hard to follow for readers with low vision or dyslexia |
| `text spacing (WCAG 1.4.12) of the style guide's` | `readability.spec.ts` | style guide, text; style guide, code blocks; style guide, tables | column widths | light | with the reader's own spacing applied, blocks overlap, an element cuts off its content, or the text runs under the footer (the code's scroller and the record viewer may scroll or clip as designed); on the text page, also when a case takes one line at that width | WCAG 1.4.12: a reader may widen the spacing, and nothing may be lost. It runs on the pages of the components that have a place in the text |
| `content layout of the style guide's text page` | `content-layout.spec.ts` | style guide, text | column widths | light | a section heading is not larger than the text; headings, paragraphs or list items overlap; the text ends less than 32 pixels above the footer; or a case takes one line at that width | The structure of the text must show at a glance, and nothing may hide under the sticky footer |
| `heading colours and underline of the style guide's text page` | `content-layout.spec.ts` | style guide, text | 1280 | light and dark | a section heading or subheading has the colour of the text; the level-4 heading is not underlined; or the text of a paragraph is | Headings stand apart from the text by colour, amber ([ADR-003](../docs/adr/ADR-003-readability-first-amber-as-accent.md)), and the level-4 heading by its underline alone |
| `numbered list of the style guide's text page` | `content-layout.spec.ts` | style guide, text | 1280 | light | a numbered list shows no numbers, or its indent or the space above it differs from a bulleted list | Both kinds of list must sit in the column the same way |
| `list markers take the amber accent` | `content-layout.spec.ts` | style guide, text | 1280 | light and dark | bullets or numbers are not amber, or the text of the items is | The markers guide the eye; the text must keep its own colour |
| `continue 4 columns deeper than their indent` | `code-blocks.spec.ts` | style guide, code blocks | 320 | light | a wrapped code line, indented with spaces or with tabs, does not continue four columns deeper than its own indent; or the nested-indentation case or the tab case has no line that wraps at 320 | In wrapped code, the reader must tell a new line from a continued one; the code counts a tab up to the next multiple of 4 and only the browser can say whether it draws it so |
| `copying the code of the cases` | `code-blocks.spec.ts` | style guide, code blocks | 320 | light | copying a code block, wrapped or not, tabs and blank line included, gives anything other than the code: the toggle's name, extra blank lines | Code copied from a page must run as written |
| `single-value table cells of` | `tables.spec.ts` | style guide, tables | 320, 393, 412, 767 | light | a table cell with no space in it, in the case of ranges with an en dash such as "63–44" or in the case of words with a hyphen such as "read-only", is split over two lines; or a case has no such cell, or is not a table, or its heading is renamed | A single value broken in two reads as two values; the cases are written so that, without the rule, the browser breaks them |

### Frame

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `header with the reader's text spacing` | `header-layout.spec.ts` | home page, with the longest titles the build accepts | every device | light | with the reader's own spacing applied, the logo, the title and the menu slot overlap, the header cuts off its content, or the page scrolls sideways | WCAG 1.4.12 for the header, which changes with the width and the title |

## Layout broken on some screens

Faults that appear only at some widths: the page scrolls sideways, a
component takes the wrong form, parts overlap.

### Integration (net)

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `no sideways scroll on the ${pageName}` | `content-layout.spec.ts` | every page of the sitemap | 320 | light | the page scrolls sideways, as it is or with the reader's spacing (tables in table form excepted) | The text breaks only at spaces, so one long word can push the page sideways: it depends on the text ([break long words](../docs/backlog/break-long-words-in-the-text.md)) |

### Component

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `each one scrolls or fits as written` | `code-blocks.spec.ts` | style guide, code blocks | column widths | light | a case of the page does not do what is written for it in the test (the long line, the nested indentation and the tabs scroll and show the wrap toggle; the short block and the blank line fit and do not), or wrapped code still scrolls, or the page scrolls sideways; or a case's heading is renamed, and the test names the case it cannot find | The toggle is offered only where it is needed, and must work |
| `exactly one form of each table` | `table-forms.spec.ts` | style guide, tables | column widths | light | both forms of a table show, or none; a table is wider than its column; the page scrolls sideways; or a case's heading is renamed, and the test names the case it cannot find | A table shows as a table while its columns fit, as a record viewer below that |
| `every record of the rows of different heights` | `table-forms.spec.ts` | style guide, tables | 320, 393, 412, 767 | light | the case of rows of different heights is not a record viewer; a row of a record is not level with its label, or not as tall; or the number row does not read "k / n" | Each value must sit beside its column's name, also when one value wraps over many lines, a label wraps, or a value is empty |
| `the form of the simple, six-column and limit tables at` | `table-forms.spec.ts` | style guide, tables | 767, 768 | light | the simple table is not a table, or the six-column table is not a table at 768 and a record viewer at 767, or the table of texts at the limit is not a table at 767 and 768 or is wider than its column | The threshold the build estimates puts each case in the form written for it where the column widens. The limit table is made only of single words, in italic, inline code and bold, so its estimate is close to its real minimum width and just under the column at 767: an estimate too low for those texts makes the table wider than its column (measurements: [card](../docs/done/2026-11-15_show-the-table-in-the-style-guide.md)) |

### Frame

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `header layout at ${width}` | `header-layout.spec.ts` | home page, with the longest titles the build accepts | every device | light | the wrong logo shows for the width, the title has the wrong size or too many lines, parts overlap or leave their place, or the page scrolls sideways | The header changes form three times with the width, and any title must fit it |
| `boxes and their date labels are laid out well` | `content-layout.spec.ts` | home page | column widths | light | at rest or under the mouse, the page scrolls sideways, a date label covers the description or the next box, or the border does not cross the middle of the label | The date sits on the box's border line, and must stay readable |
| `boxes are 2.25em apart` | `content-layout.spec.ts` | home page | column widths | light | the boxes are not 2.25 times the text size apart, or the first has a margin above | The list's own spacing must win over the column's rule for lists |
| `at rest the box has a 2px border` | `home.spec.ts` | home page | 1280 | light and dark | a box's border is not 2 pixels, solid, amber, with 8-pixel rounded corners | The look agreed for the boxes |
| `hover and keyboard focus raise the border` | `home.spec.ts` | home page | 1280 | light | under the mouse or the keyboard focus the border does not reach 4 pixels, or the title moves | The reader sees which box is active, without the text jumping |

## Broken navigation or pages

Faults that leave a reader on a dead end: a link that leads nowhere, a
page without its style, an address the site does not answer.

### Frame

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `loads stylesheet, fonts and favicon` | `home.spec.ts` | home page | 1280 | light | the header, the main text or the footer is missing, or the stylesheet, a font or the favicon is not served | The smallest proof that the site is up and whole |
| `lists the articles, newest first` | `home.spec.ts` | home page | 1280 | light | the list of articles is not newest first, or an entry lacks its description or date, or the list has bullets | The home page is where readers find the articles. It still names the two articles, a content fact ADR-004 meant to remove |
| `from the home page, the title` | `home.spec.ts` | home page, one run per article | 1280 | light | an article's title does not open that article | Each entry of the list must lead to its article. It names the articles, as above |
| `the date comes last, after the description` | `home.spec.ts` | home page | 1280 | light | in an entry the date is not the last element | The date is the label on the box's border, read after the description |
| `a click on the description of an entry` | `home.spec.ts` | home page | 1280 | light | a click on the description does not open the article | The whole box leads to the article, not just its title |
| `a click on the lower half of the date label` | `home.spec.ts` | home page | 1280 | light | a click on the lower half of the date does not open the article | The label sits on the border, half outside the box, and its lower half must still lead there |
| `the footer link of the home page` | `home.spec.ts` | home page | 1280 | light | the footer link does not lead to the about page | The footer is how a reader learns the terms of reuse |
| `from the about page, the text links` | `home.spec.ts` | about page | 1280 | light | a link in the text does not lead to the licence page or to the curriculum | The links between the information pages |
| `the logo of the licence page leads home` | `home.spec.ts` | licence page | 393, 1280 | light | the logo, by click or by keyboard, does not lead home; or both its forms show; or its focus is not visible | The logo is the way home from every page, in both its forms |
| `page has its title, heading and stylesheet, and leads home` | `not-found.spec.ts` | "page not found" (build only) | 1280 | light | the page lacks its title, heading or style, or its link does not lead home | A reader on a wrong address must find the way back |
| `the footer link of the "page not found"` | `not-found.spec.ts` | "page not found" (build only) | 1280 | light | its footer link does not lead to the about page | Its footer links are written for any depth of address |
| `a missing address, however deep` | `not-found.spec.ts` | a made-up address (live only) | 1280 | light | the host does not answer with status 404, the "page not found" and its style, or its footer link breaks | Only the host decides what a missing address shows, so it is checked live |
| `the breadcrumb leads to the index and marks the current page` | `style-guide-navigation.spec.ts` | style guide, code blocks | 320, 1280 | light | the breadcrumb is not a navigation named "Breadcrumb", or is not between the header and the text, or its link to the guide does not lead to the index, or the current page is a link or is not marked as current, or the "›" is not hidden from screen readers; or the index has the breadcrumb or "Back to top" | From a page of the guide the reader must find the way back to the list, and the screen reader must hear where they are |
| `Back to top, by keyboard, brings the view and the next Tab to the top` | `style-guide-navigation.spec.ts` | style guide, code blocks | 320, 1280 | light | "Back to top" is missing, is not below the text and above the footer, or after Enter the view is not at the top or the next Tab does not reach the logo, the page's first focusable element | A keyboard reader sent back to the top must also start from there, not from where the link was |

## A wrong face to search engines and shared links

Faults that make a page found badly, or shown badly when its link is
shared.

### Integration (net)

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `every page of the sitemap meets Lighthouse` | `seo.spec.ts` | every page of the sitemap, in one test | Lighthouse's own | — | a rule of Lighthouse's SEO category neither passes nor is not applicable | The rules search engines publish, checked by the tool they recommend ([card](../docs/done/2026-11-07_check-seo-with-lighthouse.md)) |
| `has one canonical link with its full address` | `canonical-link.spec.ts` | every page of the sitemap | 1280 | light | a page has no canonical link, more than one, or one with another address | Search engines count one address per page, not the copies |
| `has its description and its Open Graph tags` | `open-graph.spec.ts` | every page of the sitemap | 1280 | light | the description is missing or over 160 characters, or the Open Graph tags are missing or disagree with the page (title, address, type, date, author) | A shared link shows its title, description and kind of page |

### Frame

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `asks not to be indexed and has no canonical link` | `canonical-link.spec.ts` | "page not found" | 1280 | light | the page does not ask not to be indexed, or has a canonical link | An error page must not end up in search results |
| `has no description and no Open Graph tags` | `open-graph.spec.ts` | "page not found" | 1280 | light | the page has a description or an Open Graph tag | An error page has nothing to share |

## A change of font that does not reach the whole site

The stylesheet names the site's fonts by role (text, code, logo) in one
place; everything else refers to the role. Card:
[give the fonts three roles in the stylesheet](../docs/wip/give-the-fonts-three-roles-in-the-stylesheet.md).

### Component

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `the stylesheet defines the three font roles and no other font` | `font-roles.spec.ts` | the built stylesheet | none | none | `style.css` has a variable or a class of Tailwind's own fonts (sans, serif, mono), or its font variables are not exactly text, code and logo | A font left in the stylesheet outside the roles would not change when a role's font changes |
| `takes the font of ${role}` | `font-roles.spec.ts` | style guide: text, tables, code blocks, fonts | 1280; 393 for the text logo | light | the font the browser computes for a paragraph, a list item, a table cell, the page title or the Literata sample differs from the one of `--font-text`; for a code block, code inside a line or the Hack sample, from `--font-code`; for the logo (characters or text) or the pixel sample, from `--font-logo` | A role that does not reach an element leaves it in another font when the font of the role changes: the style guide would show a font the site does not use |
| `the three font roles are three different fonts` | `font-roles.spec.ts` | style guide, fonts | 1280 | light | two roles compute to the same font | The comparisons above would pass vacuously if two roles were the same font |

## Tests that would say nothing

Faults in the tests themselves: a shortcut the suite takes that stops
being valid, so passing tests would no longer prove anything.

### Guard

| Find in files | File | Pages | Widths | Themes | Fails when? | Why |
|---|---|---|---|---|---|---|
| `the dark and the light theme differ only in colours` | `themes.spec.ts` | every built page (build only) | bands | both, compared in one test | any element has a different size or position in the dark theme than in the light one | Tests not about colours run in the light theme only; that is valid only while the themes differ in colours alone |
| `the README lists every browser test once` | `readme.spec.ts` | none: it reads the code and this file (build only) | — | — | a test has no row in this file or more than one, a row's text is not found exactly once in `tests/`, in the file it names, on a line that declares a test | This file is useful only while it is true; a rule alone depends on someone remembering it |
