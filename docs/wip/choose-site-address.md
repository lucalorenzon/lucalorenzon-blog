# Choose the site's public address

- **Opened:** 2026-10-26
- **Status:** decided on 2026-11-01, configuration not done yet.

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

## Decisions

The user owns both `lucalorenzon.it` and `lucalorenzon.com`.

**A custom domain, not the GitHub Pages sub-path.** The site may later
move to another host, possibly a European cloud provider rather than an
American one. An own domain keeps the public address independent of where
the site is deployed, so such a move changes no shared link.
Rejected: keeping the sub-path, because every public URL would change at
the first move.

**Both domains reach the site, `lucalorenzon.com` is the main one.** For
now the content is the same for both. If the project goes well, only the
`.com` is meant to last, for the author's professional presence; the `.it`
stays for personal email. The `.it` still matters today: the only email
the author can give as a reference is on `lucalorenzon.it`, so people
looking for the site will likely type that domain first.
GitHub Pages serves one custom domain per site (it pairs a domain with
its `www` form, nothing more), so the second domain has to redirect to the
first.

**`www.lucalorenzon.com` is the main address; `lucalorenzon.com` redirects
to it.** A `www` name points to the host through an alias (a `CNAME`
record), which keeps working when the host changes its numeric addresses;
a bare domain cannot have an alias and must point to fixed addresses,
which many cloud hosts and delivery networks do not guarantee. GitHub
calls the `www` form "the most stable type of custom domain". It survives
a move better, which is the reason for having an own domain at all.
Rejected: `lucalorenzon.com` as main address, shorter but tied to fixed
addresses.

**The `.it` redirects to `https://www.lucalorenzon.com` with Aruba's
redirect service.** Aruba, the `.it` registrar, installs an HTTPS
certificate on the redirected domain by itself; nothing has to be written
or maintained, and the `.it` stays outside GitHub, consistent with the
first decision. Limit: the path is not kept, so
`www.lucalorenzon.it/anything` lands on the home page. It matters little,
because the `.it` is not advertised: it serves people who type the domain
by hand. Rejected:
- an alias (`CNAME`) from `www.lucalorenzon.it` to `www.lucalorenzon.com`.
  An alias only changes which server is contacted, not the address the
  browser asks for: GitHub receives a request for `www.lucalorenzon.it`,
  a name no site of its own carries. A simulated request on 2026-11-01
  got `404 Not Found` from `Server: GitHub.com` over HTTP and a
  certificate name mismatch over HTTPS;
- a second GitHub Pages site serving only a redirect page: it gets a
  certificate too, but GitHub Pages cannot answer with a real redirect,
  only with a page that redirects, and it would tie the `.it` to GitHub.

The `.it` email (Google, with its `MX`, SPF and DKIM records) must not be
touched: only the records of the site's names change.

Sources: GitHub Docs, "About custom domains and GitHub Pages" and
"Managing a custom domain for your GitHub Pages site"; Aruba guide,
"Come funziona il redirect".

## When to pick this up

Before the first increment that adds a second page or any internal link,
or earlier if the `residuality` review
(`docs/done/2026-10-28_challenge-skeleton-with-residuality.md`) points at it.
