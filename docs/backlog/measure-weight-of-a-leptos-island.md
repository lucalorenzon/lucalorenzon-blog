# Measure what a Leptos island weighs

- **Opened:** 2026-11-05, by the user, while choosing what a wide table
  does on a phone, in the card
  [let the Markdown ask for components for wide code and tables](../done/2026-11-05_annotate-markdown-for-wide-content.md);
  in Later.
- **Status:** noticed, not started. To be picked up with the first
  island the site needs.

## Why

The site renders its pages with Leptos at build time and sends no
script. Leptos was chosen also to allow interactive parts later, as
"islands": small components compiled to WebAssembly and brought to
life in the browser, while the rest of the page stays static HTML.
The user wishes for several such components.

An island has a cost the static pages do not have: the WebAssembly
and the code that loads it, downloaded with the page. The user worries
about the weight of the page, and wants to know how much of it is the
reactive machinery, paid once and shared by every island, and how much
belongs to each component.

The question first came up for the record viewer of wide tables,
which could have needed a script to keep its labels aligned. It
turned out to work with HTML and CSS only (spike A in that card), so
the tables no longer need an island, and this measurement waits for
the first component that does.

## Proposal

A spike, timeboxed to one hour, as planned in the tables card: a
throwaway project outside the repository with Leptos 0.8 islands,
built as this site is built, with no server; first an empty island,
then one doing real work. Measured: the compressed size of the
WebAssembly and of its loader in both cases, so the shared part and
the component's part can be told apart, and whether islands work
without a server.

## Known from the start

- **The build changes:** the first island adds a WebAssembly target
  and its build step to `scripts/build-site.sh` and to both workflows.
- **The weight is checked by** the
  [performance check in the build](check-performance-in-the-build.md),
  once it exists.
