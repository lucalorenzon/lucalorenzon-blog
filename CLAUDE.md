# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Personal blog for Luca Lorenzon ("Lvk@73r"), built with the [Leptos](https://github.com/leptos-rs/leptos) Rust web framework (v0.8, stable toolchain, with the standard `islands` feature — migrated from `experimental-islands`/nightly per ADR-001, 2026-08-20) and server-rendered via `actix-web`. Styling is Tailwind CSS v4 compiled by `cargo-leptos`/`dart-sass`. This started from the `leptos-rs/start` template, since diverged (see `README.md`).

## Commands

Requires Rust **stable** (pinned via `rust-toolchain.toml`) and the `wasm32-unknown-unknown` target, plus `cargo-leptos` and `dart-sass` installed.

- **Dev server with hot reload**: `cargo leptos watch` — serves at `http://localhost:3000`
- **Production build**: `cargo leptos build --release`
- **Run built server standalone**: after a release build, the binary is at `target/release/blog_start` and needs `target/site` alongside it (see env vars `LEPTOS_OUTPUT_NAME`, `LEPTOS_SITE_ROOT`, `LEPTOS_SITE_PKG_DIR`, `LEPTOS_SITE_ADDR` — set in the `Dockerfile`)
- **End-to-end tests (Playwright)**: `cargo leptos end-to-end` (runs `npx playwright test` inside `end2end/`, per `end2end-cmd`/`end2end-dir` in `Cargo.toml`). To run a single spec directly: `cd end2end && npx playwright test tests/example.spec.ts`
- **Docker build**: `docker build -t blog_start .` — multi-stage build (`rust:1-bookworm` builder targeting `x86_64-unknown-linux-musl` for a static binary, `scratch` runtime)

There is no separate lint/format command configured beyond standard `cargo fmt` / `cargo clippy`.

## Architecture

### Compilation targets (cfg-gated via feature flags)

The crate compiles to three different targets depending on enabled features (`Cargo.toml`):
- `ssr` (actix-web server binary, `src/main.rs`) — pulls in `actix-web`, `actix-files`, `leptos_actix`
- `hydrate` (WASM client bundle, `src/lib.rs` `hydrate()` entrypoint) — hydrates only the interactive **islands**, not the whole page
- `csr` (pure client-side rendering, unused in normal dev/prod flow — only relevant for tools like Tauri)

`cargo-leptos` builds both the `ssr` binary and the `hydrate` WASM bundle from the same crate in one pass (see `[package.metadata.leptos]` in `Cargo.toml`: `bin-features = ["ssr"]`, `lib-features = ["hydrate"]`).

### Islands architecture

This app uses Leptos's standard `islands` feature (migrated from `experimental-islands` per ADR-001): pages are server-rendered by default (no JS shipped), and only components explicitly marked `#[island]` are hydrated with WASM on the client. Plain `#[component]` functions stay static HTML. When adding interactivity (state, `on:click`, signals), the component must be an `#[island]`, not a `#[component]` — see `DynamicHeader` (`src/components/headers/headers.rs`) and `LightDarkSwitch` (`src/components/menu/menu.rs`) for the pattern, including the `is_browser()` guard needed because islands' setup code still runs during SSR.

### Routing & page structure

- `src/app.rs` — the `App` root component: sets up `<Html>`/`<Meta>`/`<Stylesheet>`/`<Body>` and `leptos_router` routes. Currently only `/` (`HomePage`) and a catch-all `NotFound` (404) exist.
- `src/layout.rs` — `Layout` is a slot-based wrapper that expects **exactly three children in order** (`ArticleTitle`, `ArticleAbstract`, `ArticleContent`); it pattern-matches on the children slice and falls back to an "Article not correctly configured" view if the shape doesn't match. Any new article page must build its content through this triplet.
- `src/components/` — one subdirectory per UI unit (`headers/`, `footers/`, `logos/`, `menu/`), each with its own `mod.rs`; components are re-exported through `src/components/mod.rs`.

### Static assets

Files under `assets/` are synced verbatim into the site output at build time (`assets-dir` in `Cargo.toml`) and served from `/assets` by the actix server (`src/main.rs`); referenced in views as absolute paths, e.g. `/assets/images/ostia_sea_top_image.webp`.

### Styling

Tailwind **v4** (CSS-first config, no `tailwind.config.js` — removed) is wired via `tailwind-input-file` in `[package.metadata.leptos]` (`Cargo.toml`), not `style-file`; `cargo-leptos` downloads the standalone `tailwindcss` CLI itself. `input.css` is the entry point: `@import "tailwindcss";`, an explicit `@source "./src/**/*.rs";` (v4 doesn't scan `.rs` by default), a `@theme` block with the custom `sm/md/lg/xl` breakpoints and `Futura`/`Bookerly`/`Menlo` font stacks, a native `@utility text-shadow*` (replacing the old v3 plugin), and `@custom-variant dark` for class-based dark mode (`dark:` variants, toggled by the `LightDarkSwitch` island). The old `@kamona/tailwindcss-perspective` plugin (v3-only, required an npm install that was never wired into this repo/Docker) was dropped — it was for an abandoned parallax/Z-axis experiment, unused in current code; v4 has native `perspective-*` utilities if that's revisited.
