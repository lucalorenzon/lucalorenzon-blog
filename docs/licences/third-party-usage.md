# Third-party skill/plugin usage log

Tracks every consultation of CC BY-NC-SA-licensed marketplace skills/plugins
in this repository, per the licensing gate in `~/.claude/CLAUDE.md`
("Third-Party Skill Licensing") and `ll-dev-setup`'s ADR-001. Installed as a
Claude Code plugin only, enabled per-project (`.claude/settings.json`,
project-scope, never in the shared `claude/settings.base.json`), used with
`claude` only.

| Date | Plugin | Version | Purpose | Conclusion |
|---|---|---|---|---|
| 2026-08-21 | `modularity@vladikk-modularity` (Vlad Khononov, CC BY-NC-SA 4.0) | 1.5.0 | Ran `/modularity:design` from scratch (fresh session, no prior context) on EP-001-UC-001-S001 (Article domain type + ContentSource port), to compare against the manually-run `/software-design` pipeline. Executed on `experiment/modularity-plugin-trial` (dormant, unmerged; raw design docs preserved there, never merged to main). | Independent convergence on the main structural decision (single `ContentSource` trait, `list_published` returning an explicit not-yet-implemented error instead of a stub). Two artifacts ported into S001's Decisions Log in our own words (never copied verbatim, per ADR-001): a unified `FetchError` enum, and a raw-string boundary DTO consumed once by `Article::new`. One real gap surfaced: neither this project's `/software-design` nor the plugin calibrate on language-specific mechanics (ownership, static/dynamic dispatch, async) unless explicitly prompted — raised as a backlog item in `ll-dev-setup` (EP-001-skill-suite-restructuring, Open Issues) for a future "language mechanics" skill/step. |
