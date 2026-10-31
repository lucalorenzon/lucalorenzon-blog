# Take the site's content from several repositories

- **Opened:** 2026-10-31, by the user, while the card
  [put the curriculum on the home page](../wip/put-curriculum-on-home-page.md)
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
