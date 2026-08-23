# ADR-003: Repository topology and hosting for GitHub Pages deployment

- **Date:** 2026-08-23
- **Status:** Proposed (decision agreed; infrastructure actions — repo visibility change, content repo creation — not yet executed)
- **Stories:** EP-001-UC-001-S004 (automatizzare-pipeline-ci-build), EP-001-UC-001-S005 (distribuire-output-gestire-fallimento-deploy)

## Context

ADR-002 already decided that articles live as markdown files in a git repository separate from the site's code repository, read through a `ContentSource` port. That decision left two things open: which repository actually serves the published website on GitHub Pages, and whether the code repository and the content repository need to be public.

Today the code repository (`lucalorenzon-blog`) is private, and GitHub Pages is not yet configured on it. GitHub's own hosting rules constrain which repository can publish a Pages site and under what account plan — a fact that has to be settled before the CI pipeline (S004/S005) can be built, and before the dedicated content repository (a prerequisite already flagged on ADR-002) is created.

A market check on git-based content tooling (Decap CMS, its actively-maintained successor Sveltia CMS, TinaCMS) confirmed that the "no CMS, git as storage" direction from ADR-002 still holds in 2026 — none of these tools require abandoning git-as-storage, they are optional editing UIs on top of it. That confirmation is recorded here because it was verified in the same session, but it does not change ADR-002; this ADR is scoped to hosting and repository topology only.

## Decision

The site is published from two repositories, not three:

- **`lucalorenzon-blog`** (this repository) — Rust/Leptos source, builds the static output, and publishes it to GitHub Pages directly via a GitHub Actions workflow (the current standard "Build with GitHub Actions" Pages source, not a `gh-pages` branch or a `/docs` folder).
- **A dedicated content repository** (name to be assigned, e.g. `lucalorenzon-blog-content`, per ADR-002) — markdown articles only. It is cloned during the code repository's CI build via a fine-grained personal access token or an SSH deploy key; it never itself serves a Pages site.

GitHub Pages' visibility constraint applies only to the repository that *serves* Pages. On a Free personal account, that repository must be public — a private repository can only serve Pages with GitHub Pro (or Team/Enterprise Cloud for organizations). This constraint does not touch the content repository at all, since Pages is never enabled on it; the content repository's visibility is a fully independent choice and can stay private.

Given that, `lucalorenzon-blog` will be made public rather than upgrading to GitHub Pro. There is no privacy need for the Rust source itself — the epic's own goal is professional visibility — so paying to keep it private would buy nothing. The content repository can remain private if desired, since its visibility carries no Pages constraint.

## Consequences

**Positive:**
- No recurring cost: GitHub Pages runs on the Free plan once the code repository is public.
- No third repository, no third credential to provision and rotate — the CI pipeline needs exactly one cross-repo credential (to read the content repo from the code repo's Actions run).
- The published website's content stays exactly as private or public as the content repository is configured, independent of the code repository's own visibility.

**Negative / Risks:**
- The Rust source becomes publicly readable. Accepted: no secrets or business-sensitive logic live in this codebase, and the epic's business outcome already assumes public visibility of the site itself.
- A fine-grained PAT or deploy key used by the Actions workflow to clone the content repository is a credential that must be stored as a repository secret and rotated/revoked if compromised — an operational responsibility that didn't exist while the content repository didn't exist yet.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Upgrade to GitHub Pro to keep `lucalorenzon-blog` private while still serving Pages from it | Recurring cost (~$4/month) for a privacy need that doesn't exist — the source has no secrets, and the epic's goal is professional visibility, not source confidentiality. |
| Three repositories: private code repo + private/public content repo + a separate public repo dedicated only to the built static output, pushed to by CI | Avoids both the Pro cost and public code exposure, but adds a third repository and a third credential (to push build output) for a privacy need not actually held. Only justified if source-code confidentiality became a real requirement later — not the case today. |
| Serve Pages from a `gh-pages` branch or `/docs` folder instead of the Actions-based Pages source | Still a single-repository decision independent of the topology question; rejected because GitHub Actions as the Pages source is the current default publishing path and integrates directly with the existing CI pipeline stories (S004/S005) without an extra branch to keep in sync. |

## Technical Notes

- GitHub Pages source: repository Settings → Pages → "Build and deployment" → "GitHub Actions" (not "Deploy from a branch").
- Cross-repo checkout: the workflow's `actions/checkout` step for the content repository needs a token with read access to it — a fine-grained PAT scoped to that single repository (stored as a repository secret, e.g. `CONTENT_REPO_TOKEN`) or an SSH deploy key added to the content repository with the corresponding private key as a secret in the code repository. `persist-credentials: false` should be set on the default checkout of `lucalorenzon-blog` itself to avoid credential conflicts between the two checkouts in the same job.
- Execution not yet performed as of this ADR: `gh repo edit lucalorenzon/lucalorenzon-blog --visibility public` (repo visibility change) and creation of the dedicated content repository. Both are follow-up actions, expected to be carried out explicitly after this ADR is agreed, and wired into the pipeline as part of S004/S005.

## References

- Stories: EP-001-UC-001-S004 (automatizzare-pipeline-ci-build, not yet written), EP-001-UC-001-S005 (distribuire-output-gestire-fallimento-deploy, not yet written)
- Related ADRs: ADR-002 (content-markdown-in-git — this ADR builds on its "separate content repository" decision without changing it)
- Epic: EP-001 (rilancio-blog-professionale)
