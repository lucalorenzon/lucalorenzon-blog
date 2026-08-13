# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Personal blog for Luca Lorenzon ("Lvk@73r"), built with the [Leptos](https://github.com/leptos-rs/leptos) Rust web framework (v0.6, nightly, with `experimental-islands`) and server-rendered via `actix-web`. Styling is Tailwind CSS compiled by `cargo-leptos`/`dart-sass`. This started from the `leptos-rs/start` template — the README still documents template-level setup, not this project.

## Commands

Requires Rust **nightly** (pinned via `rust-toolchain.toml`) and the `wasm32-unknown-unknown` target, plus `cargo-leptos` and `dart-sass` installed.

- **Dev server with hot reload**: `cargo leptos watch` — serves at `http://localhost:3000`
- **Production build**: `cargo leptos build --release`
- **Run built server standalone**: after a release build, the binary is at `target/server/release/leptos_start` and needs `target/site` alongside it (see env vars `LEPTOS_OUTPUT_NAME`, `LEPTOS_SITE_ROOT`, `LEPTOS_SITE_PKG_DIR`, `LEPTOS_SITE_ADDR` — set in the `Dockerfile`)
- **End-to-end tests (Playwright)**: `cargo leptos end-to-end` (runs `npx playwright test` inside `end2end/`, per `end2end-cmd`/`end2end-dir` in `Cargo.toml`). To run a single spec directly: `cd end2end && npx playwright test tests/example.spec.ts`
- **Docker build**: `docker build -t leptos_start .` — multi-stage build using `rustlang/rust:nightly-bullseye`

There is no separate lint/format command configured beyond standard `cargo fmt` / `cargo clippy`.

## Architecture

### Compilation targets (cfg-gated via feature flags)

The crate compiles to three different targets depending on enabled features (`Cargo.toml`):
- `ssr` (actix-web server binary, `src/main.rs`) — pulls in `actix-web`, `actix-files`, `leptos_actix`
- `hydrate` (WASM client bundle, `src/lib.rs` `hydrate()` entrypoint) — hydrates only the interactive **islands**, not the whole page
- `csr` (pure client-side rendering, unused in normal dev/prod flow — only relevant for tools like Tauri)

`cargo-leptos` builds both the `ssr` binary and the `hydrate` WASM bundle from the same crate in one pass (see `[package.metadata.leptos]` in `Cargo.toml`: `bin-features = ["ssr"]`, `lib-features = ["hydrate"]`).

### Islands architecture

This app uses Leptos's `experimental-islands` feature: pages are server-rendered by default (no JS shipped), and only components explicitly marked `#[island]` are hydrated with WASM on the client. Plain `#[component]` functions stay static HTML. When adding interactivity (state, `on:click`, signals), the component must be an `#[island]`, not a `#[component]` — see `DynamicHeader` (`src/components/headers/headers.rs`) and `LightDarkSwitch` (`src/components/menu/menu.rs`) for the pattern, including the `is_browser()` guard needed because islands' setup code still runs during SSR.

### Routing & page structure

- `src/app.rs` — the `App` root component: sets up `<Html>`/`<Meta>`/`<Stylesheet>`/`<Body>` and `leptos_router` routes. Currently only `/` (`HomePage`) and a catch-all `NotFound` (404) exist.
- `src/layout.rs` — `Layout` is a slot-based wrapper that expects **exactly three children in order** (`ArticleTitle`, `ArticleAbstract`, `ArticleContent`); it pattern-matches on the children slice and falls back to an "Article not correctly configured" view if the shape doesn't match. Any new article page must build its content through this triplet.
- `src/components/` — one subdirectory per UI unit (`headers/`, `footers/`, `logos/`, `menu/`), each with its own `mod.rs`; components are re-exported through `src/components/mod.rs`.

### Static assets

Files under `assets/` are synced verbatim into the site output at build time (`assets-dir` in `Cargo.toml`) and served from `/assets` by the actix server (`src/main.rs`); referenced in views as absolute paths, e.g. `/assets/images/ostia_sea_top_image.webp`.

### Styling

Tailwind is configured in `tailwind.config.js` to scan `./src/**/*.rs` for class usage (not `.html`/`.tsx`). `input.css` is the Tailwind entry point; `cargo-leptos` compiles it to `target/site/pkg/*.css` per the `style-file` setting. Custom theme additions: `sm/md/lg/xl` breakpoints, `Futura`/`Bookerly`/`Menlo` font stacks, a `text-shadow` plugin, and `@kamona/tailwindcss-perspective`. Dark mode is class-based (`dark:` variants), toggled by the `LightDarkSwitch` island.
