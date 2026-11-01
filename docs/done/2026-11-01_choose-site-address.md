# Choose the site's public address

- **Opened:** 2026-10-26
- **Status:** done on 2026-11-01. The site is served at
  `https://www.lucalorenzon.com/`.

## Why

The site was live at the standard GitHub Pages address,
`https://lucalorenzon.github.io/lucalorenzon-blog/`, which serves it
under a sub-path (`/lucalorenzon-blog/`). The choice of address was
postponed on 2026-10-26 (`docs/done/2026-10-26_put-working-skeleton-live.md`,
"Which public address?") because a single page that refers to its
stylesheet by a relative path works the same under any address.

It stops being free once pages link to each other: every internal link
and asset path then has to work under the address chosen, and every
address shared publicly stays in circulation. It was picked up once the
site showed real content (the curriculum), so the address is indexed by
search engines from the first real content on.

## Answers

The user owns both `lucalorenzon.it` (registered with Aruba, which also
manages its records; the email on it goes through Google) and
`lucalorenzon.com` (registered with Squarespace, formerly Google
Domains). Before the change, neither served a site: the `.it` showed
Aruba's parking page, the `.com` had no records.

### A custom domain, or the GitHub Pages address?

**Chosen: a custom domain.** The site may later move to another host,
possibly a European cloud provider rather than an American one. An own
domain keeps the public address independent of where the site is
deployed, so such a move changes no shared link. The cost the card had
weighed, buying a domain, no longer applied.

Rejected: keeping the GitHub Pages address. Nothing to configure, but
every public address would change at the first move, and every generated
link would have to account for the sub-path.

### Which domain is the main one?

**Chosen: both domains reach the site, and `.com` is the main one.** For
now the content is the same for both. If the project goes well, only the
`.com` is meant to last, for the author's professional presence; the `.it`
stays for personal email. The `.it` still matters today: the only email
the author can give as a reference is on `lucalorenzon.it`, so people
looking for the site will likely type that domain first.

GitHub Pages serves one custom domain per site (it pairs a domain with
its `www` form, nothing more), so the second domain has to redirect to the
first. Search engines index the domain that serves the site, not the one
that redirects, so the main domain is also the one that builds a
reputation in search results.

Open, not needed now: whether the `.com` stays a personal site (the author
as a consultant) or something else, if a company is ever founded. A
redirect can be changed later; addresses people have saved cannot.

### `www.lucalorenzon.com` or `lucalorenzon.com`?

**Chosen: `www.lucalorenzon.com` is the main address; `lucalorenzon.com`
redirects to it** (GitHub does this by itself). A `www` name points to the
host through an alias (a `CNAME` record), which keeps working when the
host changes its numeric addresses; a bare domain cannot have an alias and
must point to fixed addresses, which many cloud hosts and delivery
networks do not guarantee. GitHub calls the `www` form "the most stable
type of custom domain". It survives a move better, which is the reason
for having an own domain at all.

Rejected: `lucalorenzon.com` as main address, shorter and cleaner in
print, but tied to fixed addresses.

### How does the `.it` reach the site?

**Chosen: Aruba's redirect service, from `lucalorenzon.it` and
`www.lucalorenzon.it` to `https://www.lucalorenzon.com`.** Aruba installs
an HTTPS certificate on the redirected domain by itself; nothing has to be
written or maintained, and the `.it` stays outside GitHub, consistent with
the first answer. Limit: the path is not kept, so
`www.lucalorenzon.it/anything` lands on the home page. It matters little,
because the `.it` is not advertised: it serves people who type the domain
by hand.

Rejected:

- An alias (`CNAME`) from `www.lucalorenzon.it` to `www.lucalorenzon.com`.
  An alias only changes which server is contacted, not the address the
  browser asks for: GitHub receives a request for `www.lucalorenzon.it`,
  a name no site of its own carries. A simulated request got
  `404 Not Found` from `Server: GitHub.com` over HTTP and a certificate
  name mismatch over HTTPS.
- A second GitHub Pages site serving only a redirect page. It gets a
  certificate too, but GitHub Pages cannot answer with a real redirect,
  only with a page that redirects, and it would tie the `.it` to GitHub.

The `.it` email (Google, with its `MX`, SPF and DKIM records) was not to
be touched: only the records of the site's names change.

### Configured by hand, or as code?

**Chosen: by hand**, in the registrars' panels and in the repository's
Pages settings. No public interface for the records was found for
Squarespace, nor a Terraform provider for Aruba's domains (Aruba publishes
one for Aruba Cloud, a separate product line); the redirect service is
configured from Aruba's panel. Only GitHub's setting could be written as
code, and keeping Terraform state and credentials for one setting costs
more than it gives.

Rejected: Terraform, for now. The user declined a card for it. If the
site moves host, the records can move to a service with a Terraform
provider by changing the domain's name servers, without changing
registrar.

## What was configured

| Where | What | By |
|---|---|---|
| GitHub account settings | `lucalorenzon.com` verified, so no other account can claim it or a subdomain | the user |
| Squarespace, records of `lucalorenzon.com` | `www` alias (`CNAME`) to `lucalorenzon.github.io`; four `A` and four `AAAA` records of GitHub Pages for the bare domain; the verification `TXT` record | the user |
| Repository Pages settings | custom domain `www.lucalorenzon.com`, HTTPS enforced | the user (the agent's attempt was blocked by its permission rules, by design) |
| Aruba, redirect service | `lucalorenzon.it` and `www.lucalorenzon.it` to `https://www.lucalorenzon.com` | the user |
| Repository | the example address in `end2end/playwright.config.ts`, the link in `docs/backlog/place-1985-photo.md`, the entry in `CHANGELOG.md` | the agent |

The deploy workflow needed no change: the live-site test takes the
address from the deployment's output (`page_url`), which now reports the
new domain. With a workflow-based deployment, Pages needs no `CNAME` file
in the repository.

Squarespace rejected the `AAAA` addresses in their short form
(`2606:50c0:8000::153`) and accepted them written in full
(`2606:50c0:8000:0000:0000:0000:0000:0153`).

## Results observed on 2026-11-01

- The browser test passed on the live site at the new address:
  `BASE_URL=https://www.lucalorenzon.com npx playwright test`, 31 of 31,
  accessibility included.
- GitHub obtained a Let's Encrypt certificate covering
  `www.lucalorenzon.com` and `lucalorenzon.com`, expiring 2027-01-30 and
  renewed by GitHub.
- Every entrance reaches `https://www.lucalorenzon.com/` with one
  permanent redirect (301): `lucalorenzon.com`, `lucalorenzon.it` and
  `www.lucalorenzon.it`, over HTTP and HTTPS, the `.it` with a valid
  certificate from Aruba.
- The `.it` email records were unchanged after the redirect was set:
  Google's seven `MX` records, the SPF record, Google's DKIM key.
- A stale answer in GitHub's delivery network, still there when this card
  was closed, about 90 minutes after the change: requests declaring
  compression (as every browser does) to `http://www.lucalorenzon.com/`
  got the `404` cached at the Rome server before the domain was set, and
  the former `github.io` address redirected to `http://` rather than
  `https://`. Requests without compression, and all HTTPS requests,
  already got the right answer. A cache entry outside the project's
  control, with no expiry declared; it is expected to expire by itself.
  Whether it did is recorded in the session minute.
