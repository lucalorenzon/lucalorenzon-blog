# Show the licences on the site

- **Opened:** 2026-11-02, by the user, from the card
  [choose the licence of the articles](2026-11-02_choose-articles-licence.md).
- **Status:** done on 2026-11-03, published with version 0.7.0.

## Why

The licences of the site's text were declared in the repository
(`content/README.md`), but a reader of the site sees only the pages,
where the footer said "© Luca Lorenzon 2026" and nothing more. With no
licence in sight, a reader can only assume all rights are reserved, the
opposite of what the author wants: the articles are meant to be reused,
translated and adapted, with a citation. Creative Commons asks for the
licence to be stated next to the work. The card came before the two
articles from Medium, so they arrive with their licence in sight.

## The answers

### Where the reader meets the licence

**Chosen: in the footer**, where readers look for a site's terms and
where the "©" already was. Rejected: at the end of the content, closer
to the work but something every kind of page would have to remember.

### What the footer says

**Chosen: "© 2026 Luca Lorenzon · Reuse with attribution"**, where
"Reuse with attribution" is a link to a licence page. The user asked
that a reader who knows nothing about licences understands from the
footer alone that the text may be reused by citing the author. The
licence names (CC BY 4.0, MIT-0) say nothing to such a reader, and the
full explanation, code samples included, does not fit a footer meant
for one line.

Rejected:

- **The licence names in the footer:** short, but meaningless to most
  readers.
- **The author's name as the only link, to an "About" page** (the
  user's first idea, after Eli Bendersky's site): the shortest, but the
  footer would no longer say that reuse is allowed, and a reader who
  sees only "©" assumes it is not. The user asked whether stating it
  openly sounds arrogant; the agent's view, which the user shared, is
  that "©" is the claim and "Reuse with attribution" the offer.
- **"Reuse with credit":** "credit" may suggest money to readers whose
  language is Italian or another Romance language.

### The licence page

**Chosen: `licence.html`, with the licences only**, in plain words:
what a reader may do with the articles, the code samples, the
curriculum and material by others, then the site's code; the official
licence names are links. The text is in `content/licence.md`, approved
by the user.

Rejected:

- **An "About" page:** it would overlap with the curriculum on the
  home page.
- **"Disclosure":** the user's choice at first, after Simon Willison
  and Martin Fowler; on both sites "Disclosures" declares conflicts of
  interest (who pays them, what they own), not licences, so the name
  would surprise an English reader. A statement such as "views are my
  own, not my employer's" would fit that name; it was left out of this
  card.

### The curriculum's exception

**Chosen: a footer of its own on the page that shows the curriculum,
"© 2026 Luca Lorenzon · This CV may be copied, not changed"**, the
user's idea. The exception sits where the rule sits, in the same form
and in plain words, without adding anything to the curriculum. Each
page now chooses its footer phrase; when the curriculum moves off the
home page, its phrase moves with it.

Rejected:

- **A vertical bar on the left with the text written vertically** (the
  user's first idea): vertical text reads only when very short, so it
  would have held licence names rather than plain words; it takes width
  from the narrow column on phones; and the space beside the text on
  wide screens belongs to the card
  [use the space beside the text on wide screens](../backlog/use-space-beside-text-on-wide-screens.md).
- **A line under the heading "Curriculum Vitae":** clear, but it sits
  above the content and interrupts it.
- **Only the general footer, with the exception explained on the
  licence page:** a reader copying the curriculum without following the
  link would not learn of it.

### How the build checks it

**Chosen: the licence page gets the same checks as the home page**, at
the user's request: unit tests on each page's footer phrase, link and
title; the accessibility checks (rules and expected accessibility tree,
also on the live site); the readability and content layout checks at
every screen size. Two of them, on roles (third-level headings) and on
consecutive paragraphs, apply only to the home page, since the licence
page has neither. Following the footer link is checked from the home
page and from the "page not found", also at a deep missing address on
the live site.

### A way back from the licence page

Seeing the page on a local server, the user found the licence page a
dead end, and asked for the logo to lead to the home page. The user
chose to do it in this card, before publishing, rather than in the card
[add site navigation](../wip/add-site-navigation.md): this card
created the dead end. Both forms of the logo are one link; a screen
reader announces it as "Lvk@73r, home page" (the agent's wording), and
the ASCII art is hidden from it. Keyboard focus is visible, and the
header's layout is unchanged at every width.

## Results

Published with version 0.7.0: build run
[41545778766](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41545778766)
and deploy run
[41545783733](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41545783733)
passed, the browser test on the live site included. On
`https://www.lucalorenzon.com/` the footer reads "© 2026 Luca Lorenzon ·
This CV may be copied, not changed"; `licence.html` answers 200; a deep
missing address still answers 404 with the site's page. Locally, 48 unit
tests and 105 browser tests passed (one skipped, the live-only check).
