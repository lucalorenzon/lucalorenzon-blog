# Changelog

What each published version of the site brought. The layout follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Rules for versions

A version is published by tagging a commit of `main` with `vMAJOR.MINOR.PATCH`
(how: [README](README.md#publish-a-version)). The publication fails if this
file has no section for that version.

What each number means for this site:

- **MAJOR:** a completely new design, or a different navigation.
- **MINOR:** an additional feature.
- **PATCH:** a security fix or a bug fix. While test articles live in this
  repository, adding one also counts as a patch.

A version tag is never moved. If one has to be deleted, its name is never
used again: the next version takes the next number. GitHub can stop a tag
from being moved, but not a deleted name from being reused, so this rule
holds only because it is written here.

Why these rules: [the card that set them](docs/done/2026-10-28_separate-build-from-deploy.md).

## [Unreleased]

## [0.1.0] - 2026-10-28

### Added

- An "under construction" home page: the first version of the site.
- Publication from a version tag: changes merged into `main` are built and
  tested, but go live only when a version is tagged.
