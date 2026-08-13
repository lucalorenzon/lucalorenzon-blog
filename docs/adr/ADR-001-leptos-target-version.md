# ADR-001: Target version for Leptos and the related dependency ecosystem

- **Date:** 2026-08-14
- **Status:** Proposed
- **Stories:** EP-001 (epic-level decision, no stories split yet)

## Context

The site has been dormant since 2024-08-14, built on Leptos 0.6.x (nightly, using the `experimental-islands` feature) and never updated since. Before any other work on the site can resume — search, deployment, redesign, content — it needs to compile and run again on dependency and toolchain versions that are still actively maintained. Waiting longer only widens the gap and increases the number of breaking changes that will need to be absorbed in one go.

## Decision

Adopt Leptos **0.8.20** — the current stable release (published 2026-06-25) — as the target version, together with the matching stable releases of the rest of the ecosystem: `leptos_meta` 0.8.6, `leptos_router` 0.8.15, `leptos_actix` 0.8.7, `leptos-use` 0.19.0, `leptos_icons` 0.7.1, `icondata` 0.7.0, `wasm-bindgen` 0.2.127, and `actix-web` 4.14.1.

A `0.9.0-beta` release exists (published 2026-07-18), but it is not yet stable. The Leptos maintainer's own status update (May 2026) describes the project as feature-complete and moving to light maintenance, with no urgency behind 0.9 — described as cleanup only, with no significant new features. Targeting a beta release for a personal project with no external delivery pressure would trade stability for no real benefit; if 0.9 stabilizes later, upgrading again from 0.8 is a normal, low-risk follow-up.

The most consequential change on the path from 0.6 to 0.8 is that the `experimental-islands` feature flag has been renamed and stabilized to `islands`, with a new hydration entry point (`hydrate_islands()`) and an `islands=true` flag required on `HydrationScripts`. `leptos_actix` continues to support islands with no material difference from the Axum-based examples in the official docs, so the current actix-web backend does not need to be replaced to complete this update — that stays out of scope, deferred to EP-003.

## Consequences

**Positive:**
- The project moves onto an actively supported, stable release line instead of a two-year-old minor version.
- Adopting `islands` (the stabilized flag) removes reliance on an experimental feature name that no longer exists in current Leptos.
- Establishes a known-good baseline dependency set that EP-002 through EP-007 can build on without re-litigating version choices.

**Negative / Risks:**
- Two minor version jumps (0.6 → 0.7 → 0.8) bundled into a single update increase the surface of breaking changes to absorb at once, compared to smaller incremental upgrades.
- `wasm-bindgen` is currently pinned exactly (`=0.2.93`); moving to 0.2.127 must be re-verified against whichever `wasm-bindgen-cli`/`cargo-leptos` version is installed locally, since mismatches between the crate and the CLI tool are a common source of build failures in this ecosystem.
- If Leptos 0.9 stabilizes soon after this work lands, a second upgrade may be needed sooner than usual — accepted, since 0.9 is described as cleanup-only and low risk.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Target Leptos 0.9.0-beta directly | Not yet stable; adopting a beta as the baseline for a project meant to stay usable going forward adds risk with no offsetting benefit — no feature in 0.9 is needed here. |
| Stay on Leptos 0.6.x, patch-bump only | Doesn't resolve the core problem: 0.6 is two release lines behind, and every future epic (EP-002..EP-007) would still be built on an outdated, effectively unsupported base. |
| Incremental upgrade path (0.6 → 0.7 → 0.8, verifying at each step) | More thorough in isolating which version introduces which breaking change, but for a single-developer project with no CI safety net yet, doing it in one pass and fixing whatever the compiler reports is faster — the failure modes to fix are the same either way. |

## Technical Notes

- Current pinned versions (`Cargo.toml`): `leptos`/`leptos_meta`/`leptos_router`/`leptos_actix` `"0.6"`, `leptos-use` `"0.12"`, `leptos_icons`/`icondata` `"0.3"`, `wasm-bindgen` `"=0.2.93"`, `actix-web` `"4.5"`.
- Target versions: `leptos`/`leptos_meta`/`leptos_router`/`leptos_actix` `"0.8"`, `leptos-use` `"0.19"`, `leptos_icons` `"0.7"`, `icondata` `"0.7"`, `wasm-bindgen` `"0.2.127"`, `actix-web` `"4.14"`.
- Feature flag rename: `experimental-islands` → `islands` (on the `leptos` crate); hydration entry point becomes `leptos::mount::hydrate_islands()`; `HydrationScripts` needs `islands=true`.
- `rust-toolchain.toml` currently pins `channel = "nightly"` with no explicit date; a newer nightly snapshot may be required depending on what 0.8 needs — to be confirmed during the build.

## References

- Stories: none yet (epic-level decision; stories to be split next)
- Related ADRs: none
- External: crates.io version history for `leptos` and related crates (queried 2026-08-14); [leptos-rs/leptos issue #4707](https://github.com/leptos-rs/leptos/issues/4707) (May 2026 status update); [Leptos Islands guide](https://book.leptos.dev/islands.html)
