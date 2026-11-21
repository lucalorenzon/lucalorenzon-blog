# Build the fonts page from the configuration

- **Created:** 2026-11-16
- **Status:** Closed
- **Closed:** 2026-11-17

## Why

The user opened this card on 2026-11-16, while cutting the card
[link the fonts to the build](link-the-fonts-to-the-build.md).

The fonts page of the style guide writes by hand the fonts it shows
(`FONTS_SHOWN` in `src/style_guide/fonts_page.rs`): their names, the
styles shown and the variant of each sample, whose names carry the
font's (`sample-literata`, `sample-hack`, `sample-pixel`). The styles
the site serves are written elsewhere, in the `@font-face` rules: the
two lists can drift, and a style added to the site would have no
sample without anything noticing. Once the configuration holds the
fonts, the page can take from it the roles, the fonts and their styles.

The user wants this card done before the style guide is published.

## What is known

- **The author's text** (`content/style-guide/fonts.md`) has one
  section per font, titled with its name, and the build stops when a
  section and a font do not match; it explains why each font was
  chosen, so a change of font also asks for a change of the text.
- **The configuration already holds what the samples need**, checked
  on 2026-11-16: `config/fonts.txt` says which font plays each role,
  and each `config/fonts/<folder>.txt` gives the font's family name and,
  for each file, the label the page shows under the sample (`.name`),
  its style and its weight. The label was added for this card.
- **The build already reads the configuration** (`src/style_guide/fonts.rs`,
  called by `write_fonts` in `src/main.rs`) and writes the roles' values
  and the `@font-face` rules to `target/generated/fonts.css`. The fonts
  page does not read it yet.
- **Each sample is a named variant of the stylesheet**
  (`style/input.css`, `sample-literata` to `sample-pixel`), written out
  in full in the Rust code because Tailwind finds the classes it must
  emit by scanning the sources
  ([ADR-001](../../adr/ADR-001-name-styles-by-visual-variant.md)). Each
  variant takes the font from its role (`font-text`, `font-code`,
  `font-logo`), not from a font's name.
- **The pixel font is drawn at 16 pixels**, its native size, where it
  does not blur, and no wider than 80 characters; the other two keep
  the size of the page's text. The configuration holds no size.

## Questions to answer

The ones known when the card was taken into work, and those that came up during it.

- **How a sample gets its style when the styles come from the
  configuration.** Today one variant of the stylesheet exists for each
  sample, named after the font. If the configuration adds a style, or a
  role changes font, a variant named after the old font would be wrong,
  and a new style would have no variant; Tailwind cannot emit a class it
  does not find written in the sources.

  **Answered by the user on 2026-11-16.** Through the role: the
  page takes from the configuration the font that plays each role, and
  shows a sample for each of that font's files.

  The variant, agreed by the user on 2026-11-16: named after the role,
  not the font (`sample-text`, not `sample-literata`), since it draws
  with the role's font whatever that is; the browser picks the font's
  file from the `@font-face` rules, so the variant only asks for the
  role, the style and the weight. The build writes one variant for
  each file of each role, named from the role, the style and the
  weight (for example `sample-text-italic-700`), next to the
  `@font-face` rules it already writes, and puts the names in the list
  of classes Tailwind reads, so a weight added to the configuration
  gets its sample with nothing written by hand. Rejected: variants
  written by hand in `style/input.css`, one per role and style, with
  the build stopping when a file has none; the stylesheet would stay
  readable in one place, but a new weight would still ask for a
  variant written by hand, which is what this card was opened to
  avoid.
- **Where the pixel font's size lives.** It is a property of that font,
  not of the logo role, yet the configuration does not hold it; if the
  logo changed font, the 16 pixels would stay.

  **Answered by the user on 2026-11-16.** In the font's description, as
  an optional key with its native size in pixels (`size=16` for IBM VGA
  8x16), a property of the font and not of the role. Only the samples
  of the fonts page read it: a font with a size is drawn there at that
  size, and its lines kept to 80 characters, as `sample-pixel` does
  today; a font without one keeps the page's size, as Literata and Hack
  do. The size is not the font's size across the site: the header draws
  the pixel font at 32 pixels for the written logo, twice its native
  size, and at 16 for the logo drawn in characters, and keeps those
  sizes written by hand; tying them to the configuration is other work,
  not this card's.
- **What happens when two roles use the same font.** The configuration
  does not forbid it, and the page has one section per font.

  **Answered by the user on 2026-11-16.** The author is free to do it,
  and explains it in the text; the font has one section, which names
  both roles (see the next question).
- **Whether the page shows which role each font plays.** Today the
  sections are titled with the font's name only, and their order is the
  author's.

  **Answered by the user on 2026-11-16.** Yes: a font is chosen
  for a role, and the rest of the site depends on the role, not on the
  font. The build writes the roles, from the configuration, under the
  section's title, so they stay true when a font moves to another role;
  a role named in the author's text is not checked. The form, proposed
  by the agent and agreed by the user on 2026-11-16: a paragraph right
  under the title, "Role: text" or "Roles: code, logo" in the roles'
  order, with a named variant of its own (`roles-line`, written by hand
  in `style/input.css`), smaller and in italics like the samples'
  labels. The agent first proposed the name `font-roles`, and corrected
  it on 2026-11-17: in Tailwind a class starting with `font-` names a
  font (`font-text` is the role's), so `font-roles` would read as a
  font called "roles". A screen reader reads it as the first line of the section,
  and the page's outline stays the title and one level-2 heading per
  font. Rejected: a level-3 heading, which would announce a subsection
  that is not there; a definition list, too much for one line; the
  roles inside the title, which would lengthen every entry of the list
  of headings.

  The order, agreed by the user on 2026-11-16: the text before the
  first placeholder stays at the top of the page, as the introduction;
  the sections follow the roles' order in `config/fonts.txt` (text,
  code, logo), a font with two roles taking the place of the first;
  inside a section come the title, the roles, the author's pieces in
  the roles' order, then the samples, one for each file.
- **Whether the page has one section per role or one per font.** Came
  up while answering the questions above: the user first meant one
  section per role, but the author's text explains the choice of a
  font, and a font may play two roles.

  **Answered by the user on 2026-11-16.** One section per distinct font
  of the configuration, titled with its family name taken from the
  configuration, naming the roles it plays and showing a sample for
  each of its files. A change of font asks for a change of the text, as
  it should: the text is where the choice of a font is explained (how,
  in the next question). Rejected: one section per role, which follows
  the structure of the site, but would split or repeat the explanation
  of a font that plays two roles. The cost of the choice, a reader
  looking for a role reads it inside the sections, is small with three
  roles.
- **How the author's text is tied to the fonts and their roles.** Came
  up with the order of the sections: today the author's text has one
  section per font, in its own order, and the build stops on a section
  that names no font.

  **Answered by the user on 2026-11-16.** The author's text is cut in
  pieces, each opened by a placeholder that names a font and a role.
  The build composes the page itself: for each role of the
  configuration it takes the piece of the pair made of the role and its
  configured font, and puts it in the font's section; a font with two
  roles gets two pieces in its one section. A pair of the configuration
  with no piece stops the build; a piece whose pair the configuration
  does not hold (a font not configured, a role the build does not
  handle) is ignored. This changes today's rule, which stops the build
  on a section that names no font, and it is safe: a misspelt
  placeholder is ignored, but then the right pair has no piece and the
  build stops anyway; and the author may keep the text of a font no
  longer used. A pair written twice stops the build (agreed by the user
  on 2026-11-16), so a piece copied and forgotten cannot end up on the
  page, or out of it, unnoticed; also a pair the configuration does not
  hold, which would otherwise be ignored: two equal pieces are a
  mistake of the text either way, and one rule is simpler to explain.
  The content's Markdown holds no HTML (a unit test checks
  it), so the placeholder cannot be an HTML comment.

  The placeholder's form, proposed by the user on 2026-11-16: a
  level-2 heading `## <folder> | <role>`, one role per heading, for
  example `## literata | text`, `## hack | code`,
  `## ibm-vga-8x16 | logo`. The font is named by its folder, as in
  `config/fonts.txt`, made of small letters, digits and hyphens only,
  so it cannot be mistaken for the separator; the role is written as
  in the configuration. The page shows instead the family name and the
  roles, written by the build. Why a heading: it is not HTML, it ends
  the piece before it, as the sections do today, and it stays readable
  in an editor or on GitHub. A level-2 heading not in this form (no
  separator, a font or a role not written in small letters, digits and
  hyphens) stops the build, agreed by the user on 2026-11-16: it is a
  mistake of writing, and ignoring it would drop its text unnoticed. A
  heading in the form whose pair the configuration does not hold is
  ignored, as said above.
- **How the browser tests tell the roles apart when two roles share a
  font.** Came up while the first step was written: the tests of the
  roles compare the font the browser computes for an element with the
  value of its role's variable, and when two roles share a font the two
  values are equal, so the comparison cannot tell them apart. The test
  `the three font roles are three different fonts` stops the build in
  that case, which the answer above allows.

  **Answered by the user on 2026-11-17.** Before measuring, the tests
  replace the three roles on the page with three made-up, different
  names, and check that each element gets its role's name: they check
  the link from the element to its role, whatever fonts the
  configuration holds. The test of the three different fonts goes; the
  cases named after a font ("the Literata sample") are named after the
  role. The real fonts stay checked by the test that every font file
  loads and by the one that checks each sample against its file.
  Rejected: skipping the tests of two roles that share a font, the
  smallest change, but a skipped test is easily missed, and the link
  from element to role would go unchecked exactly then.

## Acceptance tests

Agreed with the user on 2026-11-16.

1. The page has one section for each distinct font of the
   configuration, titled with its family name, in the roles' order; a
   font with two roles has one section, in the place of its first
   role. Unit tests: one on the real configuration, one where the code
   and the logo share a font.
2. Under each title, the build writes the roles' line, "Role: …" or
   "Roles: …, …", with the variant `roles-line`. Unit test.
3. The text before the first placeholder stays at the top of the page;
   each piece goes in its font's section, in the roles' order; no
   placeholder appears on the page. Unit tests.
4. The build stops, with a message naming the file and the heading,
   when a pair of the configuration has no piece, when a pair is
   written twice (held by the configuration or not), and when a
   level-2 heading is not in the form `## <folder> | <role>`. One unit
   test per case.
5. A well-formed piece whose pair the configuration does not hold does
   not appear on the page and does not stop the build. Unit test.
6. Each file of a font has a sample, labelled with its `.name`, drawn
   by a variant the build writes and names from the role, the style
   and the weight (for example `sample-text-italic-700`); a file added
   to the configuration, for example of weight 600, gets its sample
   with no other change. Unit tests, one on a configuration with one
   more file.
7. The key `size` is optional; a value that is not a positive whole
   number stops the build; a font with a size has its samples at that
   size and their lines at most 80 characters wide, a font without one
   keeps the page's size. Unit tests.
8. In the browser, each sample is drawn with the right file: the same
   family, style and weight as the file it stands for. Browser test on
   the fonts page, with its row in `end2end/README.md`.
9. Nothing is left written by hand: `FONTS_SHOWN` and the variants
   `sample-literata` to `sample-pixel` are gone; the author's text uses
   the placeholders, its words unchanged apart from the headings.
   Review of the code.
10. With today's configuration the page looks as it does now, apart
    from the roles' line; screenshots of the real page, on a phone and
    a laptop screen, side by side with today's, shown to the user
    before the commit.
11. Every check of the pipeline passes: formatting, clippy, unit tests,
    type check and browser tests, run locally as in `build.yml`.
12. Added on 2026-11-17, from the answer on the tests of the roles:
    the browser tests of the roles pass with made-up names in place of
    the three roles' fonts, with no test asking for three different
    fonts, and no case named after a font; their rows in
    `end2end/README.md` follow.

## Recap

Done in two steps, each with a commit of its own; every check of the
pipeline passed before each.

- **The samples from the configuration** (`ccbe164`): the pixel font's
  native size in its description (`size=16`); the build writes one
  variant per file of each role's font (`sample-<role>-<style>-<weight>`)
  and lists the names for Tailwind with `@source inline(...)` in
  `target/generated/fonts.css`; `FONTS_SHOWN` and the six variants
  written by hand are gone; a browser test checks each sample against
  its file. The page looked the same, pixel by pixel, on a phone and a
  laptop screen.
- **The placeholders and the roles** (`1e5cb31`): the author's text cut
  by `## <folder> | <role>`, found with the converter's own parser, so
  an indented or underlined level-2 heading is seen too; the sections
  composed by the build with the roles' line; the browser tests of the
  roles run on made-up fonts, and the test asking for three different
  fonts is gone. The only visible change is the roles' line. Checks:
  411 unit tests, 270 browser tests passed, one skipped (the "page not
  found" test that runs on the live site only).

## Cautions

- A change of font in `config/fonts.txt` stops the build until
  `content/style-guide/fonts.md` has a piece `## <folder> | <role>` for
  it: on purpose, the text explains the choice.
- The values measured on a font (table widths, the wrap toggle's
  threshold) do not follow a change of font yet: see "What it leaves".

## What it leaves

- The values measured on a font (table widths, the wrap toggle's
  threshold), taken again when it changes: card
  [measure the fonts again when they change](measure-the-fonts-again-when-they-change.md),
  next in the plan.

## References

- [Link the fonts to the build](link-the-fonts-to-the-build.md):
  the card this one was cut from; it put the fonts in the configuration.
- [Show the fonts in the style guide](show-the-fonts-in-the-style-guide.md):
  the card that built the fonts page as it is today.
- [Measure the fonts again when they change](measure-the-fonts-again-when-they-change.md):
  cut from the same card, done after this one.
- [Build a design system](build-a-design-system.md): the card in
  work this one belongs to; the style guide is published after it.
