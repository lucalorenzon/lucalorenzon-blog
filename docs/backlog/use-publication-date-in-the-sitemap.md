# Use the publication date in the sitemap

- **Opened:** 2026-11-07, by the user, while choosing which git date
  the sitemap uses ([card](../done/2026-11-07_add-sitemap.md)).
- **Status:** noticed, not started.

## Why

The sitemap's date of a page's last change comes, for now, from the
author's date of the last commit that changed its Markdown file. That is
the day the change was written, not the day the page changed on the
site: the page changes online when the version that carries the change
is published, often days later. The user chose the author's date for
now and this card for the publication date.

## Known from the start

- **An example of the gap:** the articles were changed on 2026-11-04
  and went online with version 0.9.0 on 2026-11-06.
- **The risk,** inferred from how Google describes the use of
  `<lastmod>`, not found documented as such: Google visits a page on
  the 5th, while it is still the old version; on the 6th it reads in
  the sitemap "last change: the 4th", may conclude that nothing changed
  since its visit, and not read the page again.
- **The tension with the snapshot:** the build that writes the sitemap
  runs on a commit of `main` before the version's tag exists, and the
  published site is exactly that snapshot, tested and not changed
  afterwards
  ([card](../done/2026-10-28_separate-build-from-deploy.md)). A
  publication date would mean writing or correcting the sitemap at
  deploy time, publishing something other than what was tested.
- **Rejected for now:** the committer's date, which a rebase changes,
  and which is no closer to the publication.

## Questions

- Whether the sitemap can be finished at deploy time, and how it is then
  tested.
- Or whether the date can be known before the tag: for example, the
  date of the version a change is planned for.
