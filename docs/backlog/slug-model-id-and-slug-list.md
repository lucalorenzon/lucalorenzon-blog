# Backlog: article identity — stable id plus a list of slugs, one active

**Origin:** surfaced 2026-10-26 while discussing EP-001-UC-001-S004 (content
validation step before page generation).
**Epic:** to be decided — slugs chosen for search keywords sit in the SEO area
that EP-001 currently lists as out of scope.
**Status:** open, not yet split into a story.

## Why

Today an article's slug *is* its identity: the slug is both the markdown file
name and the page URL. That ties two things that change for different reasons.
A page should keep its identity while its public address changes — for
example to target different search keywords, or to fix a poorly chosen URL —
and old addresses should keep working by redirecting to the current one.

## Goal

- Every article has a stable **id**, independent of its URL.
- Every article has a **list of slugs**, exactly **one of them active**.
- Inactive slugs redirect to the active one (how: see
  [redirect-handling.md](redirect-handling.md)).
- Interim view, agreed 2026-10-26: changing the active slug is treated as
  generating a new page and retiring the old one behind a redirect to the new
  one.

## Validation rules this implies

Checked once over the whole content set, before page generation:

- ids are unique across all articles;
- every slug is unique across all articles, active and inactive alike;
- every article has exactly one active slug.

## Detail — what it changes in existing code

- **Contract change on closed stories.** EP-001-UC-001-S001 (`Article` domain,
  `ContentSource`) and EP-001-UC-001-S002 (slug uniqueness) are built on
  "slug = identity": `FilesystemContentSource::get_by_slug` opens
  `{slug}.md`, and `ensure_slug_is_unique` answers "is this one candidate
  already taken?", not "are all slugs in the set distinct?". This needs its
  own story (or a reopening of S001/S002), not absorption into S004.
- The frontmatter already carries a `slug` field separate from the file name,
  so two files can declare the same slug today and nothing catches it.
- Open point raised during the discussion: whether alternative slugs actually
  help search ranking. If they redirect, search engines mostly treat them as
  old URLs pointing at the canonical one; if they served the same page
  without redirecting, they would be duplicate content. This does not weaken
  the case for the model as a way to handle URL changes over time — only the
  "activate keywords" motivation is to be validated.
