# Decide whether the logo is a component of its own

- **Created:** 2026-11-22
- **Status:** Open

## Why

Asked by the user on 2026-11-22, at the end of the session that took
the card [review the logo](review-the-logo.md) into work, and cut from
that card by the user the same day. The logo is today part of the page
frame (`src/page_frame.rs`), shared by every page. The style guide
tests each component once, on its own page
([build a design system](build-a-design-system.md)); a logo of its own
would have its own page, and the frame's page
([show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md))
would show it only as a part.

## What is known

- **The new logo** is an icon, a photo cut to a circle, with the name
  in Literata bold italic beside it, both standing on the header's
  line, which belongs to the frame. Since 2026-11-22 the line is also
  hidden under the name, like a thread
  ([hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md)).
- **The text column was made a component and went back into the
  frame**
  ([make the text column a component](make-the-text-column-a-component.md),
  [decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md)):
  a precedent on what being a component costs here.

## Questions to answer

The ones known when the card was written; there may be more.

- **What the logo would gain as a component**: a page of its own in the
  style guide, its tests apart from the frame's, reuse elsewhere.
- **What ties it to the frame**: the header's line it stands on, and
  the room it shares with the title.

**Answered for now by the user on 2026-11-22, with the agent**, on the
card
[show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
which asked the same question for the frame's parts: not yet. The site
is static HTML, and the behaviours the user expects soon (a header
that is transparent and turns opaque when the text passes under it,
parallax, the title and the logo's name moving when the title reaches
the header) tie the logo, the title and the header together. They are
expected to give the components their shape; cut now, the logo would
be cut from the wrong information. The card stays open, so the
question comes back with those behaviours.

**Confirmed by the user on 2026-11-22**, when the card came up in Now:
nothing is done now, and the card goes to Later. When the header's
behaviours are built, the question will come up by itself.

## Dependencies

- The card [review the logo](review-the-logo.md), which builds the new
  logo inside the frame; closed on 2026-11-22.
- **The header's dynamic behaviours**, which have no cards yet: the
  question is taken up again when they are built.

## References

- [Review the logo](review-the-logo.md): the card this one was cut
  from.
- [Build a design system](build-a-design-system.md): one page per
  component.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the page that would show the logo as a part of the frame.
- [Hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md):
  an effect that ties the name to the frame's line.
- [Make the text column a component](make-the-text-column-a-component.md)
  and
  [decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md):
  the precedent of the text column.
