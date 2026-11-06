# Take the site's content from several repositories

- **Opened:** 2026-10-31, by the user, while the card
  [put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md)
  was being worked on.
- **Status:** noticed, not started.

## Why

The site's content is written elsewhere: the curriculum in its own
repository, the articles in other repositories. For the first increment
the curriculum's site version is copied into this repository by hand,
because fetching it during the build would bring in permissions and
credentials (the curriculum's repository is private), too much for that
card.

As more sources arrive, copying by hand will not scale: each copy can
drift from its source, and nothing tells when it has.

## Questions

- How the build gets each source: download, git submodule, a package, a
  release file? Which of them work with a private repository, and with
  which credentials?
- Who cleans a source of personal data before it is published: the
  source repository, or this build?
- How the site tells which version of each source it shows.
- How the home page's date of last change is found, for the sitemap.
  Noted by the user on 2026-11-06, while making the sitemap
  ([card](../wip/add-sitemap.md)): the home page changes when a new
  article is added, which is a matter of how content is edited and
  arrives on the site. Until then the home page is in the sitemap
  without a date.
