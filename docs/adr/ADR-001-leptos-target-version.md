# ADR-001: Target version for Leptos and the related dependency ecosystem

- **Date:** 2026-08-14
- **Status:** Accepted (implemented 2026-08-20 — `cargo leptos build` succeeds on the target versions and stable toolchain)
- **Stories:** EP-001 (epic-level decision, no stories split yet)

## Context

The site has been dormant since 2024-08-14, built on Leptos 0.6.x (nightly, using the `experimental-islands` feature) and never updated since. Before any other work on the site can resume — search, deployment, redesign, content — it needs to compile and run again on dependency and toolchain versions that are still actively maintained. Waiting longer only widens the gap and increases the number of breaking changes that will need to be absorbed in one go.

## Decision

Adopt Leptos **0.8.20** — the current stable release (published 2026-06-25) — as the target version, together with the matching stable releases of the rest of the ecosystem: `leptos_meta` 0.8.6, `leptos_router` 0.8.15, `leptos_actix` 0.8.7, `leptos-use` 0.19.0, `leptos_icons` 0.7.1, `icondata` 0.7.0, `wasm-bindgen` (range `0.2`, currently resolving to 0.2.127), and `actix-web` 4.14.1.

A `0.9.0-beta` release exists (published 2026-07-18), but it is not yet stable. The Leptos maintainer's own status update (May 2026) describes the project as feature-complete and moving to light maintenance, with no urgency behind 0.9 — described as cleanup only, with no significant new features. Targeting a beta release for a personal project with no external delivery pressure would trade stability for no real benefit; if 0.9 stabilizes later, upgrading again from 0.8 is a normal, low-risk follow-up.

The most consequential change on the path from 0.6 to 0.8 is that the `experimental-islands` feature flag has been renamed and stabilized to `islands`, with a new hydration entry point (`hydrate_islands()`) and an `islands=true` flag required on `HydrationScripts`. `leptos_actix` continues to support islands with no material difference from the Axum-based examples in the official docs, so the current actix-web backend does not need to be replaced to complete this update — that stays out of scope, deferred to EP-003.

**`wasm-bindgen` moves from an exact pin to a range.** The dependency is currently pinned exactly (`=0.2.93`) because the crate version must match whatever `wasm-bindgen-cli` is installed locally, or the build fails outright. The target moves to a range (`"0.2"`) instead of re-pinning exactly at `0.2.127`, accepting the small risk that a future `cargo update` could drift the resolved crate version away from the installed CLI — to be caught by the build itself (AT-EP-001 "Build" table), not guarded against structurally.

**The Rust toolchain moves from nightly to stable.** Leptos 0.6 required nightly; as of 0.7/0.8, nightly is no longer required at all — it only remains available as an opt-in `nightly` feature flag on the `leptos` crate, which unlocks calling signals as functions (`count()` instead of `count.get()`) via an `Fn` trait implementation that stable Rust cannot support. The codebase currently uses this call syntax in exactly one place (`src/components/menu/menu.rs`, the `LightDarkSwitch` island, `dark_mode_enabled()`), the only signal in the project. Given the migration cost is a single call site, and nightly Rust is otherwise a source of compiler instability with no remaining technical justification for this project, the toolchain moves to stable: `rust-toolchain.toml` is updated (or removed) accordingly, the `nightly` feature flag is dropped from `leptos`/`leptos_meta`/`leptos_router` in `Cargo.toml`, and the one signal call site is rewritten to `.get()`.

## Consequences

**Positive:**
- The project moves onto an actively supported, stable release line instead of a two-year-old minor version.
- Adopting `islands` (the stabilized flag) removes reliance on an experimental feature name that no longer exists in current Leptos.
- Establishes a known-good baseline dependency set that EP-002 through EP-007 can build on without re-litigating version choices.

**Negative / Risks:**
- Two minor version jumps (0.6 → 0.7 → 0.8) bundled into a single update increase the surface of breaking changes to absorb at once, compared to smaller incremental upgrades.
- Relaxing `wasm-bindgen` from an exact pin to a range reintroduces the risk of a crate/CLI version mismatch on a future `cargo update`, which is a common source of build failures in this ecosystem — accepted in exchange for not having to manually re-pin on every future bump.
- Moving to stable Rust means losing the function-call signal syntax project-wide; low actual cost today (one call site), but any future code written against nightly examples/tutorials will need translating to `.get()`/`.set()` calls.
- If Leptos 0.9 stabilizes soon after this work lands, a second upgrade may be needed sooner than usual — accepted, since 0.9 is described as cleanup-only and low risk.
- `cargo audit` reports one unresolved advisory (RUSTSEC-2026-0258, `h2` 0.3.27, unbounded empty DATA frames) pulled in transitively via `actix-http` 3.13.3 — the latest `actix-web` 4.x line has no `h2` 0.4.x path available; a fix would require a future `actix-web` major version. Accepted as a known, unresolvable-at-this-layer risk for now (site not yet in production).
- Discovered during implementation: `leptos_meta` 0.8's SSR pipeline patches head content and `<html>`/`<body>` attributes into the streamed response by locating literal `<head>`/`</head>`/`<html`/`<body` markers — this codebase never rendered them (`app.rs` only had `<Html>`/`<Body>`/`<Meta>` "virtual" components, no literal document shell), which is a silent no-op in 0.6 but a hard panic in 0.8 (`you are using leptos_meta without a </head> tag`) on the very first request. Fixed as part of this update by adding a `shell()` function in `app.rs` (literal `<html><head>` with `<AutoReload/>`, `<HydrationScripts islands=true islands_router=true/>`, `<MetaTags/>`) and wiring it into `leptos_routes()` in `main.rs`. As a side effect, islands are now actually wired up to hydrate client-side (`<leptos-island>` markers and the hydration script are present in server output, verified against a live request), which was silently not happening before.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Target Leptos 0.9.0-beta directly | Not yet stable; adopting a beta as the baseline for a project meant to stay usable going forward adds risk with no offsetting benefit — no feature in 0.9 is needed here. |
| Stay on Leptos 0.6.x, patch-bump only | Doesn't resolve the core problem: 0.6 is two release lines behind, and every future epic (EP-002..EP-007) would still be built on an outdated, effectively unsupported base. |
| Incremental upgrade path (0.6 → 0.7 → 0.8, verifying at each step) | More thorough in isolating which version introduces which breaking change, but for a single-developer project with no CI safety net yet, doing it in one pass and fixing whatever the compiler reports is faster — the failure modes to fix are the same either way. |
| Keep `wasm-bindgen` pinned exactly at `=0.2.127` | Would keep the build deterministic and safe from crate/CLI drift, but requires manually bumping the pin on every future `wasm-bindgen` patch release; the range trades a small, build-time-visible risk for less ongoing maintenance. |
| Stay on nightly Rust with the `nightly` feature flag | Keeps the ergonomic signal call syntax with zero code changes, but nightly is no longer required by Leptos 0.8 or by the islands architecture — there is no remaining technical justification for keeping it, and it is a known source of occasional compiler instability. |

## Technical Notes

- Current pinned versions (`Cargo.toml`): `leptos`/`leptos_meta`/`leptos_router`/`leptos_actix` `"0.6"` (with `features = ["nightly"]` on `leptos`, `leptos_meta`, `leptos_router`), `leptos-use` `"0.12"`, `leptos_icons`/`icondata` `"0.3"`, `wasm-bindgen` `"=0.2.93"`, `actix-web` `"4.5"`.
- Target versions: `leptos`/`leptos_meta`/`leptos_router`/`leptos_actix` `"0.8"` (no `nightly` feature), `leptos-use` `"0.19"`, `leptos_icons` `"0.7"`, `icondata` `"0.7"`, `wasm-bindgen` `"0.2"`, `actix-web` `"4.14"`.
- Feature flag rename: `experimental-islands` → `islands` (on the `leptos` and `leptos_meta`/`leptos_router` crates); on `leptos_actix` the equivalent feature is named `islands-router`, not `islands`. Hydration entry point becomes `leptos::mount::hydrate_islands()`; `HydrationScripts` needs `islands=true` — implemented via the `shell()` function added to `app.rs` (see Consequences).
- `leptos`'s own `Cargo.toml` feature also needs `islands-router` (not just `islands`), matching `leptos_actix`. `leptos_actix`'s `islands-router` feature transitively enables `tachys/mark_branches`, which controls whether the SSR renderer's `bo-TypeId`/`bc-TypeId` hydration-boundary comments get *skipped* during client hydration. Since `leptos_actix` is only linked into the `ssr` build (not `hydrate`/wasm), setting `islands-router` only on `leptos_actix` left `mark_branches` on for the server and off for the client — every island hydrated onto a stray comment node instead of its real root element, an "Unrecoverable hydration error" on first load. Fixed by also setting `islands-router` on `leptos` itself in `Cargo.toml`, so `mark_branches` is consistent across both build targets.
- Toolchain: `rust-toolchain.toml` currently pins `channel = "nightly"` with no explicit date; moves to `stable` (or the file is removed, letting the project use whatever stable toolchain is active). The `nightly` feature flag is dropped from `leptos`, `leptos_meta`, and `leptos_router` in `Cargo.toml`.
- The single nightly-only call site to migrate: `src/components/menu/menu.rs`, `LightDarkSwitch` island — `dark_mode_enabled()` (and the corresponding call inside the `Show when=` closure) become `dark_mode_enabled.get()`.

## References

- Stories: none yet (epic-level decision; stories to be split next)
- Related ADRs: none
- External: crates.io version history for `leptos` and related crates (queried 2026-08-14); [leptos-rs/leptos issue #4707](https://github.com/leptos-rs/leptos/issues/4707) (May 2026 status update); [Leptos Islands guide](https://book.leptos.dev/islands.html); [leptos-rs/leptos discussion #4561](https://github.com/leptos-rs/leptos/discussions/4561) (nightly vs. stable, confirms nightly is optional as of 0.7/0.8, only gates the function-call signal syntax)
