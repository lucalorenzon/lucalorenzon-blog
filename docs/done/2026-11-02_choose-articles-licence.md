# Choose the licence of the articles

- **Opened:** 2026-10-28, from the light pass of `residuality`
  ([card](2026-10-28_challenge-skeleton-with-residuality.md)).
- **Status:** done on 2026-11-02.

## Why

The whole repository was under the MIT licence (`LICENSE`, `Cargo.toml`).
MIT fits the code, but articles kept in the same repository would be
under MIT too: a licence written for software, which asks to keep a
copyright notice rather than to cite the author. On 2026-10-28 the user
chose to keep MIT for the time being and to decide before the first
article; on 2026-11-02 the card came first in Now, before the card
[add the two articles already on Medium](../backlog/add-the-two-medium-articles.md).

## The answers

| What | Licence |
|---|---|
| The text of the articles, and their images made by the author | Creative Commons Attribution 4.0 International (CC BY 4.0) |
| The code samples in the articles | MIT No Attribution (MIT-0) |
| The curriculum | Creative Commons Attribution-NoDerivatives 4.0 International (CC BY-ND 4.0) |
| The site's code | MIT, as before |

All of it "unless otherwise noted": quotations and images by other
authors keep their own terms. Declared in `content/README.md`, with the
MIT-0 text in `content/LICENSE-MIT-0` and a pointer from `README.md`.

### What a reader may do with an article

**Chosen: CC BY 4.0.** The user writes to pass on what they know, to
open a discussion, or to tell a story, and hopes the articles are
reused: what they gain is being known for their value, not money. So
reuse is welcome, commercial use included; adaptations and translations
are welcome; whoever adapts may keep their own version closed. The one
condition is to be cited as the author.

The user's worry about a distorted adaptation is met, as far as a
licence can, by what CC BY 4.0 already asks: changes must be stated,
nobody may imply the author endorses them, and the author may ask for
their name to be removed from an adaptation. The moral right to the
integrity of the work is not licensed, and Italian law keeps it with
the author. A reader who gives the text a meaning the author did not
intend cannot be stopped; the user accepted it, recalling Barthes'
"death of the author".

Rejected:

- **Keep MIT:** written for software; it asks to keep a notice, not to
  cite the author, and not to state changes. It protects worst the one
  thing the user asks for.
- **CC BY-SA 4.0:** it would force whoever adapts to keep their version
  open; the user does not want to impose it.
- **Forbid adaptations (CC BY-ND) for the articles:** the only way to
  rule out distortion by contract, but it would also forbid the
  translations the user welcomes.

A licence covers the author's text, not the ideas in it: an article on
someone else's concepts is still the author's own text, and the
concepts are cited out of honesty, not because a licence asks.

### What a reader may do with the code samples

**Chosen: MIT-0.** Under CC BY 4.0, a reader pasting ten lines into a
client's project would have to cite the author beside them: friction
exactly where reuse is most hoped for, and Creative Commons itself
advises against its licences for software. The user wants a citation
for code to be a courtesy, not a condition. MIT-0 is close to the
repository's MIT and widely recognised.

Rejected:

- **Apache 2.0:** asks for a copy of the licence and a notice on every
  changed file, the heaviest of the options for a snippet; its patent
  grant is of no use here.
- **MIT:** asks to keep the copyright notice, so the citation stays a
  condition.
- **0BSD, CC0:** same effect as MIT-0; 0BSD is less known, and CC0 is a
  waiver of all rights, which Italian law does not allow in full.

### What a reader may do with the curriculum

**Chosen: CC BY-ND 4.0.** The curriculum describes the author: adapting
it would not distort an idea, but could credit the author with
experience that is not theirs, or take theirs. The user wants it copied
and passed on, also for work, and not changed. Changes of format (to a
PDF, into a form) stay allowed; translations shared with others do not.
The user will publish the curriculum in more languages themselves.

Copies taken before 2026-11-02 keep the MIT licence they came with.

### How code and text are told apart

**Chosen: a licence declaration in `content/`, and one sentence for the
whole site.** The repository follows the fonts' precedent: the licence
sits beside what it covers, and a `README.md` says what covers what.
The Creative Commons texts are linked rather than copied, which the
licences accept.

The user asked how other blogs do it before going further. Of six
widely read personal blogs, four state only "©" or nothing, which means
all rights reserved: readable and quotable, not reusable. The two that
state a licence (Eli Bendersky, Chris Wellons) use a single sentence
for the whole site, "unless otherwise noted". The user chose the same:
the curriculum is the "otherwise noted".

Showing the licences on the pages, which a reader of the site needs
since they do not see the repository, became a card of its own:
[show the licences on the site](2026-11-03_show-licences-on-the-site.md).
This card closes with the decisions and the repository's files.

## Sources

- [CC BY 4.0, legal code](https://creativecommons.org/licenses/by/4.0/legalcode.en):
  changes stated (3(a)(1)(A)(ii)), no endorsement (2(a)(6)), moral
  rights not licensed (2(b)(1)).
- [Creative Commons, FAQ](https://creativecommons.org/faq/): against
  CC licences for software; mark clearly which parts are under which
  licence.
- [MIT-0, SPDX](https://spdx.org/licenses/MIT-0.html).
- Blogs looked at on 2026-11-02:
  [Martin Fowler](https://martinfowler.com/),
  [Julia Evans](https://jvns.ca/),
  [Simon Willison](https://simonwillison.net/),
  [Dan Luu](https://danluu.com/),
  [Eli Bendersky](https://eli.thegreenplace.net/pages/about),
  [Chris Wellons](https://nullprogram.com/).
