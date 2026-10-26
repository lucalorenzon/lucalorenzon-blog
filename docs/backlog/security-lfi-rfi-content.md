# Backlog: security check against local/remote file inclusion in content

**Origin:** requested 2026-10-26 while discussing EP-001-UC-001-S004.
Absorbs residue #4 ("HTML sanitization") from S003's Stressor Analysis,
which until now lived only in the 2026-09-01 replan inventory (session
memory / Hindsight) and in no repository document.
**Epic:** to be decided.
**Status:** open, own story.

## Why

Articles come from a separate content repository and are turned into pages
by this code. Anything an article declares — an image path, raw HTML inside
the markdown, a link to a remote resource — ends up either read from disk at
build time or embedded in a page served to visitors. A mistake or a hostile
edit in the content repo must not be able to read files outside the content
folders (LFI), pull in remote code or content (RFI), or inject scripts into
the site (XSS).

## Goal

- Content can reference only files inside its own allowed folders.
- Generated pages contain no raw HTML or remote resource the build did not
  explicitly allow.
- Violations fail the content pipeline with a readable message, they are not
  silently dropped.

## Detail — findings in the current code (2026-10-26)

- **LFI, image paths.** `ImagePath::parse`
  (`src/domain/value_objects/image_path.rs`) only rejects blank values. A
  frontmatter `image: ../../something` is accepted and joined onto the images
  folder by `FilesystemContentSource::image_exists`
  (`src/adapters/secondary/content_source/filesystem.rs`). Slugs are not
  affected: `Slug::parse` already restricts them to lowercase kebab-case.
- **RFI / XSS, markdown body.** `markdown_to_html`
  (`src/pages/view_model.rs`) uses pulldown-cmark with default options, which
  pass raw HTML through unchanged; the output is inserted with `inner_html`
  in `ArticlePage`. `<script>`, `<iframe src="http://…">` or remote
  resources in an article reach the page as-is.
- Related, separate check planned on the content repository side: every
  link, image and stylesheet reference must be reachable and below a
  configurable size limit (raised 2026-10-26, not yet tracked on its own).
