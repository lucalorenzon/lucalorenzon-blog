# Choose the site's public address

- **Opened:** 2026-10-26
- **Status:** open decision, not blocking yet.

## Why

The site is live at the standard GitHub Pages address,
`https://lucalorenzon.github.io/lucalorenzon-blog/`, which serves it
under a sub-path (`/lucalorenzon-blog/`). The alternative is a custom
domain. The choice was postponed on 2026-10-26 (`docs/done/2026-10-26_put-working-skeleton-live.md`,
"Which public address?") because a single page that refers to its stylesheet by a
relative path works the same under either address.

It stops being free once pages link to each other: every internal link
and asset path then has to work under the address chosen. Deciding before
that point avoids reworking links later.

## Options

**Keep the GitHub Pages sub-path.**
- Pro: nothing to buy, configure or renew; already working.
- Con: the address is longer and tied to GitHub; every generated link must
  account for the sub-path; moving to a custom domain later changes every
  public URL already shared.

**Use a custom domain** (the previous attempt of this project had
mentioned `lucalorenzon.it` / `lucalorenzon.com`; ownership to be
confirmed by the user).
- Pro: a short, professional address independent of the hosting, which
  fits the site's purpose of showcasing the author's work; pages are
  served from the root, so links need no sub-path handling.
- Con: DNS records to set and maintain, the domain to renew; Pages needs
  the custom domain configured and HTTPS enforced; the live-site test
  must check the new address.

## When to pick this up

Before the first increment that adds a second page or any internal link,
or earlier if the `residuality` review
(`docs/backlog/challenge-skeleton-with-residuality.md`) points at it.
