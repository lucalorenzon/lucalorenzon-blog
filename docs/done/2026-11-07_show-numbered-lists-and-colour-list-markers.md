# Show numbered lists, and colour the markers of lists

- **Opened:** 2026-11-07, by the user, after the work was done: the card
  was forgotten when the problem was noticed, so it was written straight
  into `docs/done/`. Not in the plan.
- **Status:** done on 2026-11-07, in commits `fbf1ec2` (the numbers) and
  `eb8249d` (the colour of the markers).

## Why

The user noticed that the numbered list at the end of the article
"After Save Event" showed no numbers: its three points read as plain
paragraphs, with no indent. It was the only numbered list in the
content, so nobody had seen it before.

## The answers

### Why the numbers were missing

The Markdown was right, and the build turned it into a numbered list
(`<ol>`) as it should. The cause was in the stylesheet: Tailwind's base
reset removes the markers and the indent from every list, and the text
column put them back only for bulleted lists (`ul`). Numbered lists had
no rule at all.

**Chosen: numbered lists take the same space above and the same indent
as bulleted lists, with plain decimal numbers.** A list should sit in
the column the same way whatever its kind. A browser test checks it,
written first and seen failing before the fix.

### The colour of the markers

Once the numbers were back, the user asked to try the amber accent on
the markers. It is the colour of the marker only, the bullet or the
number. The shape of the lists and the colour of the items' text stay as
they were.

This fits ADR-003 (amber is an accent, not the colour of the text), but
the light page has a constraint. Amber has three tones there, and a
number is normal-size text, so it needs the strongest contrast the site
checks for text. Only the tone used for code inside a sentence
(`accent-code`, a dark brown) reaches it. A bullet is a graphic, like
the page's lines, and the lighter tone of lines and frames
(`accent-line`) is enough for it. On the dark page every tone is the
same amber, so the choice only shows on the light page.

**Chosen, by the user: bullets in the tone of lines and frames, numbers
in the tone of code inside a sentence.** The user chose it after
comparing screenshots on the real pages, on a phone and a laptop. The
bullets read as amber, matching the page's lines, and the numbers keep
the contrast of text. The cost: on the light page the two kinds of
list have slightly different shades.

Rejected:

| Option | Why rejected |
|---|---|
| Both markers in the tone of code inside a sentence | The two kinds of list would match, but on the light page the bullets turned almost black and the accent nearly disappeared |
| Numbers inside a circle, as badges | Not what the user meant: they wanted to change the colour only, not the shape. A light number on a filled amber circle would also miss the contrast |
| Markers left in the colour of the text | The user wanted to try the accent, and the screenshots showed it does not hurt the reading |

## Results

- The numbered list at the end of "After Save Event" shows its numbers,
  indented like the bulleted lists.
- Bullets and numbers are amber in both themes. The text of the items
  keeps its colour.
- Two browser tests in `end2end/tests/content-layout.spec.ts` check the
  outcome (rules 7 and 8), in the light and in the dark theme. Both
  failed before their change and pass after it.
- Every check of the pipeline passes: formatting, lint, 199 Rust
  tests, the build, the type check, and 853 browser tests (2 skipped,
  on the "page not found", unrelated).
- `CHANGELOG.md` lists both changes under "Unreleased": the numbers
  under "Fixed", the colour under "Changed".
- ADR-003 lists where amber is used, and the markers of lists are not
  in that list yet.
