# Show the first box of the home page at mid-height of the screen

- **Opened:** 2026-11-07, by the user, right after the card
  [frame each article on the home page in a box](2026-11-07_frame-home-page-articles-in-boxes.md);
  in Now.
- **Status:** closed on 2026-11-07, tried and not adopted. The home
  page stays as it was: the boxes start right below the page title.

## Why

The first thing a reader should meet on the home page is the first
thing to read, and it should meet it where the eye goes first: the
middle of the screen. The boxes start right below the page title, at
the top of the screen; on a laptop most of the screen below them stays
empty. The user wanted the first box to appear at mid-height of the
screen when the page opens, with scrolling working as usual. Across
the page the box is already centred, since the text column is.

## The answer (2026-11-07)

**Chosen, by the user: keep the page as it was.** Compared side by side
on the screen a reader sees on opening, footer included, the page with
the box lower down looked much worse than today's.

What was tried, with the page title and the header left at the top:

| Version | How | First box, from the page title (small phone / tall phone / laptop) | Second article on opening, small phone |
|---|---|---|---|
| Today | As built | 40 / 40 / 70 pixels | Partly seen |
| The middle of the first box at mid-height | Needs a script: a stylesheet cannot know the box's height, which changes with the text; the site has no script so far | 114 / 266 / 274 pixels | Partly seen |
| The top of the first box at mid-height | A stylesheet alone: the space above the list grows to half the screen's height | 191 / 329 / 328 pixels | Barely seen |

Screens: a small phone (320 by 568 pixels), a tall phone (390 by 844)
and a laptop (1280 by 800), all with the sticky footer as a reader
sees it.

- **The fear about phones did not hold.** On the small phone the
  second article is only partly seen today too, and stays so with the
  middle of the box at mid-height.
- **The trial moved the header's line.** On a laptop the header has a
  line below the logo and the title; the trial made the space by
  growing the header, so the line went down with the box. The user
  wants the line to stay at the top; a build would have needed the
  space elsewhere.

## Open point

The user feels something is not right with the home page, and that it
may not be the height of the boxes on the screen. What it is, is not
known yet; no card is opened until it can be named, so this trial is
not repeated.
