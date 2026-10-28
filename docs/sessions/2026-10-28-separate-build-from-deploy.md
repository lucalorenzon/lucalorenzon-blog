# Session 2026-10-28 — Publish from a version tag, with a CHANGELOG

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Haiku 5.5 (git, `gh` commands, watching runs) and Sonnet 5.5
(workflow code).
**Goal:** settle and carry out the card
[separate the build from the deploy, publish from a tag](../done/2026-10-28_separate-build-from-deploy.md).
Technical details and reasons are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Keep build, tag-triggered deploy and CHANGELOG in one card, not split | user | agent | — |
| 2 | Two paths: publish the `snapshot` of the tagged commit if there is one, otherwise rebuild | user | agent | — |
| 3 | Version tags created by hand, from the terminal | user | agent | — |
| 4 | `vMAJOR.MINOR.PATCH` with a meaning for each number; a test article is a patch | user | agent | — |
| 5 | Only tags on `main` publish; any other makes the deploy fail visibly | user | agent | — |
| 6 | Version tags cannot be moved; only the administrator deletes them; a deleted name is never reused (written rule) | user | agent | — |
| 7 | `CHANGELOG.md` in Keep a Changelog layout, by hand, checked by the deploy | user | agent | — |
| 8 | A GitHub Release per version, with the CHANGELOG text and the site's zip | user | agent | — |
| 9 | Documentation-only changes: the build keeps skipping them, the deploy never does | user | agent | — |
| 10 | Four steps; the build stops publishing in the same step the deploy from a tag starts | user | agent | — |
| 11 | GitHub settings changed by the agent with `gh`, `main` removed from the Pages environment | user | agent | — |
| 12 | The deploy calls `build.yml` instead of keeping its own copy of the build | agent | — | user |
| 13 | First version `v0.1.0` | user | agent | — |
| 14 | The deploy grants write permission to the called build, rather than a separate snapshot workflow | agent | — | user |
| 15 | The Release is created right after publishing, whatever the live test says | user | agent | — |
| 16 | No card for a rollback for now | user | agent | — |
| 17 | No version made only to test the publication; checked at the next real version | user | agent | — |

## Discussion

1. **Split or not.** The agent noted that the card held several decisions
   and could be split. The user kept it whole: the three parts only have
   value together. → decision 1
2. **Which site the deploy publishes.** The agent's first wording ("how
   the deploy gets the site") was not clear to the user, who described
   their own picture: a build that tests, a deploy that publishes, and
   ideally a saved zip of the build, possibly in GitHub Releases. The
   agent confirmed Releases fit, and named a tension: the latest snapshot
   may belong to a newer, unfinished commit. The user proposed to rebuild
   when the tag is not on the latest commit; the agent sharpened the test
   to "was the snapshot built from the tagged commit", and named the cost
   of a rarely used fallback. The user kept two paths. → decision 2
3. **Versions and articles.** The user gave each version number a meaning.
   The agent pointed out that adding an article fitted none of them; the
   user first said the version does not change, the agent showed that
   this left articles no way to go live, and the user settled it: test
   articles here are patches, real ones will live in another repository,
   a matter for a future card. → decisions 3, 4
4. **Guards on tags.** The user chose that tags outside `main` fail
   visibly, and that tags stay fixed with an administrator able to delete
   them. The agent noted that deleting and recreating equals moving; the
   user accepted a written rule. → decisions 5, 6
5. **CHANGELOG and Releases.** The user wanted the CHANGELOG checked but
   asked how the preparation commit fits; the agent explained the deploy
   never commits and the author tags a commit that already has the
   section. The user then asked for a Release per version with the zip.
   → decisions 7, 8, 9
6. **Order of the work and the `main` rule.** The agent proposed four
   steps and the order constraint; the user approved and had the agent
   change the settings. The user did not see why `main` should leave the
   Pages environment, fearing nothing could be published; the agent
   explained that the environment checks what started the run (a tag),
   not where the commit lives. The agent chose to have the deploy call
   the build, which keeps the rarely used fallback exercised on every
   push, and told the user. → decisions 10, 11, 12
7. **First version.** The agent recalled the `0.x` convention for a
   project still being built; the user chose `v0.1.0` and pushed the tag
   from this session's terminal, after asking whether `git push --tags`
   was the same (it is today; naming the tag pushes only that one).
   → decision 13
8. **Snapshot permissions.** Calling the build forced the deploy to grant
   it write permission; the agent kept that over a third workflow and
   told the user. → decision 14
9. **Release timing and rollback.** The user wanted the Release right
   after publishing, since a failed live test only says what to fix in a
   patch; the user raised rollback and then judged it not worth a card
   now. → decisions 15, 16
10. **Verifying the rest.** The agent noted that checking the snapshot
    path and the Release would need a version with no change for
    visitors; the user chose to wait for the next real version.
    → decision 17

## Outcome

- Commits: `aee294d` card to `wip`, `b552132` answers recorded, `efe3ff6`
  build and deploy split with `CHANGELOG.md` (step 2), `794eb5f`
  CHANGELOG prepared for 0.1.0, `4e2116e` snapshot (step 3), `e4da19f`
  Release per version (step 4), then the commit closing the card and
  this minute.
- GitHub settings: the `github-pages` environment accepts tags `v*` and
  no longer `main`; rulesets "Version tags cannot be moved" and "Only
  admins delete version tags".
- `v0.1.0` published from its tag (run
  [40641876615](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40641876615)),
  every job green, including the browser test on the live site.
- Pushes to `main` no longer publish; the build keeps the `snapshot`
  Release up to date.

## Open points

- The session minute of 2026-10-27 links the card at its old backlog
  path; minutes are not edited, so the link stays broken.
- GitHub warns that `ubuntu-latest` moves to Ubuntu 26; no failure so
  far, no card opened.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Check the snapshot path, the version's Release and, if cheap, the
  deploy's refusals at the next real version (decision 17): card
  [verify the publication at the next version](../backlog/verify-publication-at-next-version.md).
